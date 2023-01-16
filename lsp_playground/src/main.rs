use std::{process::Stdio, time::Instant};

use serde_json::json;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter},
    process::{ChildStderr, ChildStdout, Command},
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

#[tokio::main]
async fn main() {
    let mut process = Command::new("dart")
        .args(["language-server"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("failed to spawn process");

    let proc_id = process.id().expect("Unable to fetch process id");
    println!("proc id: {}", proc_id);

    let writer = BufWriter::new(process.stdin.take().expect("Failed to open stdin"));
    let reader = BufReader::new(process.stdout.take().expect("Failed to open stdout"));
    let stderr = BufReader::new(process.stderr.take().expect("Failed to open stderr"));

    let (tx_client, mut rx) = unbounded_channel::<Vec<u8>>();
    let (tx, rx_client) = unbounded_channel::<Vec<u8>>();

    let wait = setup_listeners(reader, writer, stderr, rx_client, tx_client);

    let request1 = json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":"file:///Users/mledwon/Projects/flutter_base_project","capabilities":{"textDocument":{"publishDiagnostics": {"relatedInformation": true}}},"trace":"off","workspaceFolders":null,"clientInfo":{"name":"My LSP Client","version":"1.0.0"}}});
    let request2 = json!({"jsonrpc":"2.0","id":1,"method":"initialized","params": {}});

    let json_bytes = serde_json::to_string(&request1).unwrap();
    let json_bytes2 = serde_json::to_string(&request2).unwrap();
    tx.send(
        format!(
            "Content-Length: {}\r\n\r\n{}\r\n",
            serde_json::to_vec(&request1).unwrap().len(),
            json_bytes
        )
        .as_bytes()
        .to_vec(),
    )
    .unwrap();

    tokio::spawn(async move {
        rx.recv().await.unwrap();
        let start = Instant::now();

        tx.send(
            format!(
                "Content-Length: {}\r\n\r\n{}\r\n",
                serde_json::to_vec(&request2).unwrap().len(),
                json_bytes2
            )
            .as_bytes()
            .to_vec(),
        )
        .unwrap();

        loop {
            let result = String::from_utf8(rx.recv().await.unwrap()).unwrap();
            if result.contains("main.dart") {
                let duration = start.elapsed();
                println!("Duration: {:?}", duration);
            }
        }
    });

    wait.await;
}

async fn setup_listeners(
    reader: BufReader<ChildStdout>,
    writer: (impl AsyncWrite + Unpin + Send + 'static),
    error_reader: BufReader<ChildStderr>,
    rx: UnboundedReceiver<Vec<u8>>,
    tx: UnboundedSender<Vec<u8>>,
) {
    let handle_recv = tokio::spawn(recv(reader, tx));

    let handle_send = tokio::spawn(send(writer, rx));

    let handle_error = tokio::spawn(recv_error(error_reader));

    tokio::select! {
       send = handle_send => println!("{send:?}"),
       recv = handle_recv => println!("{recv:?}"),
       handle_error = handle_error => println!("{handle_error:?}"),
    };
}

async fn recv_error(mut reader: BufReader<ChildStderr>) {
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await.unwrap();
        panic!("Error: {}", line);
    }
}

async fn recv(mut reader: BufReader<ChildStdout>, tx: UnboundedSender<Vec<u8>>) {
    loop {
        let mut buffer = String::new();
        let result = recv_server_message(&mut reader, &mut buffer).await;
        println!("Received: {}", result);
        tx.send(result.as_bytes().to_vec()).unwrap();
    }
}

async fn send(mut writer: (impl AsyncWrite + Unpin), mut rx: UnboundedReceiver<Vec<u8>>) {
    loop {
        match rx.recv().await {
            Some(data) => {
                send_to_server(&mut writer, &data[..]).await;
            }
            None => {
                println!("rx_recv: quitting loop");
                break;
            }
        }
    }
}

async fn send_to_server(writer: &mut (impl AsyncWrite + Unpin), data: &[u8]) {
    println!("Sending: {}", String::from_utf8_lossy(data));
    println!("Content-lenght: {}", data.len());
    writer.write_all(data).await.unwrap();
    writer.flush().await.unwrap();
}

async fn recv_server_message(reader: &mut BufReader<ChildStdout>, buffer: &mut String) -> String {
    let mut content_length = None;
    loop {
        buffer.truncate(0);
        if reader.read_line(buffer).await.unwrap() == 0 {
            panic!();
        };

        // debug!("<- header {:?}", buffer);

        if buffer == "\r\n" {
            // look for an empty CRLF line
            break;
        }

        let header = buffer.trim();

        let parts = header.split_once(": ");

        match parts {
            Some(("Content-Length", value)) => {
                content_length = Some(value.parse().unwrap());
            }
            Some((_, _)) => {}
            None => {
                // Workaround: Some non-conformant language servers will output logging and other garbage
                // into the same stream as JSON-RPC messages. This can also happen from shell scripts that spawn
                // the server. Skip such lines and log a warning.

                // warn!("Failed to parse header: {:?}", header);
            }
        }
    }

    let content_length = content_length.unwrap();

    //TODO: reuse vector
    let mut content = vec![0; content_length];
    reader.read_exact(&mut content).await.unwrap();
    let msg = std::str::from_utf8(&content).unwrap();

    msg.to_string()
}

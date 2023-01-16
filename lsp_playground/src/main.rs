use std::process::Stdio;

use serde_json::json;
use tokio::{
    io::{AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter},
    process::{ChildStderr, ChildStdout, Command},
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

#[tokio::main]
async fn main() {
    let mut process = Command::new("dart")
        .args([
            "language-server",
            "--protocol=analyzer",
            "--protocol-traffic-log=/Users/mledwon/Projects/just_code/logs.txt",
        ])
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

    let (tx_client, mut _rx) = unbounded_channel::<Vec<u8>>();
    let (tx, rx_client) = unbounded_channel::<Vec<u8>>();

    let wait = setup_listeners(reader, writer, stderr, rx_client, tx_client);

    let request1 =
        json!({"id":"1","method":"server.setSubscriptions","params":{"subscriptions":["STATUS"]}});
    // let request2 = json!({"id":"2","method":"analysis.setAnalysisRoots","params":{"included":["/Users/mledwon/Projects/flutter_base_project"],"excluded":[]}});

    let json_bytes = serde_json::to_vec(&request1).unwrap();
    tx.send(json_bytes).unwrap();

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
        let mut line = String::new();
        reader.read_line(&mut line).await.unwrap();
        println!("Received: {}", line);
        tx.send(line.as_bytes().to_vec()).unwrap();
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

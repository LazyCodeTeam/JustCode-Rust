import * as cdk from 'aws-cdk-lib';
// import * as cognito from 'aws-cdk-lib/aws-cognito'
import { Construct } from 'constructs';
// import { Duration, RemovalPolicy } from 'aws-cdk-lib';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as ecs from 'aws-cdk-lib/aws-ecs';
import * as ecr from 'aws-cdk-lib/aws-ecr';
import * as elbv2 from 'aws-cdk-lib/aws-elasticloadbalancingv2';


export class JustCodeStackDev extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const vpc = ec2.Vpc.fromLookup(this, "VPC", { isDefault: true });
    const cluster = new ecs.Cluster(this, 'CodeServiceClusterDev', { vpc });

    cluster.addCapacity('CodeServiceAutoScalingGroup', {
      instanceType: ec2.InstanceType.of(ec2.InstanceClass.T4G, ec2.InstanceSize.SMALL),
      machineImage: ecs.EcsOptimizedImage.amazonLinux2(ecs.AmiHardwareType.ARM),
      machineImageType: ecs.MachineImageType.BOTTLEROCKET,
    });


    const repo = ecr.Repository.fromRepositoryName(this, 'CodeSeriviceRepository', 'code_service');
    const taskDefinition = new ecs.Ec2TaskDefinition(this, 'CodeServiceTaskDef');
    const container = taskDefinition.addContainer('code_service', {
      // image: ecs.ContainerImage.fromEcrRepository(repo, process.env.CODE_SERVICE_TAG),
      image: ecs.ContainerImage.fromRegistry("amazon/amazon-ecs-sample"),
      memoryLimitMiB: 2048,
      environment: {
        PORT: '80'
      }
    });

    container.addPortMappings({
      containerPort: 80,
      hostPort: 8080,
      protocol: ecs.Protocol.TCP
    });

    const service = new ecs.Ec2Service(this, "CodeServiceDevService", {
      cluster,
      taskDefinition,
    });

    const lb = new elbv2.ApplicationLoadBalancer(this, 'CodeServiceLoadBalancerDev', {
      vpc,
      internetFacing: true
    });
    const listener = lb.addListener('PublicListener', { port: 80, open: true });

    listener.addTargets('ECS', {
      port: 8080,
      targets: [service.loadBalancerTarget({
        containerName: 'code_service',
        containerPort: 80
      })],
    });
    // const pool = new cognito.UserPool(this, 'Pool', {
    //   accountRecovery: cognito.AccountRecovery.EMAIL_ONLY,
    //   autoVerify: {
    //     email: true,
    //   },
    //   // email: cognito.UserPoolEmail.withSES({
    //   //   fromEmail: 'noreply@flutterlerneo.com',
    //   //   fromName: 'Flutter lerneo',
    //   //   replyTo: 'support@flutterlerneo.com',
    //   // }),
    //   email: cognito.UserPoolEmail.withCognito("support@flutterlerneo.com"),
    //   enableSmsRole: false,
    //   passwordPolicy: {
    //     minLength: 8,
    //     requireLowercase: false,
    //     requireUppercase: false,
    //     requireDigits: false,
    //     requireSymbols: false,
    //     tempPasswordValidity: Duration.days(3),
    //   },
    //   userPoolName: 'JustCodeDevUserGroup',
    //   standardAttributes: {
    //     email: {
    //       required: true,
    //       mutable: true,
    //     },
    //     nickname: {
    //       required: true,
    //       mutable: false,
    //     },
    //     profilePicture: {
    //       required: false,
    //       mutable: true,
    //     }
    //   },
    //   selfSignUpEnabled: true,
    //   userVerification: {
    //     emailSubject: 'Verify your email for our awesome app!',
    //     emailBody: 'Thanks for signing up to our awesome app! Your verification code is {####}',
    //     emailStyle: cognito.VerificationEmailStyle.CODE,
    //   },
    //   signInCaseSensitive: false,
    //   removalPolicy: RemovalPolicy.DESTROY,
    // });

    // const client = pool.addClient('AppClient', {
    //   authFlows: {
    //     userPassword: true,
    //
    //   },
    //   generateSecret: true,
    // })
    //
    // const domain = pool.addDomain('Domain', {
    //   cognitoDomain: {
    //     domainPrefix: 'justcode',
    //   }
    // });
    //
    // domain.signInUrl(client, {
    //   redirectUri: 'https://example.com',
    // })
  }
}

import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as ecs from 'aws-cdk-lib/aws-ecs';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as ecr from 'aws-cdk-lib/aws-ecr';
import * as ecsp from 'aws-cdk-lib/aws-ecs-patterns';
import * as elbv2 from 'aws-cdk-lib/aws-elasticloadbalancingv2';

export class CodeServiceDev {
  constructor(scope: Construct, vpc: ec2.Vpc) {
    const repo = ecr.Repository.fromRepositoryName(scope, 'CodeSeriviceRepository', 'code_service');
    const cluster = new ecs.Cluster(scope, 'CodeServiceClusterDev', {
      vpc,
    });

    cluster.addCapacity('DefaultAutoScalingGroupCapacity', {
      instanceType: ec2.InstanceType.of(ec2.InstanceClass.T2, ec2.InstanceSize.MICRO)
    });

    new ecsp.ApplicationLoadBalancedEc2Service(scope, 'CodeServiceDev', {
      cluster,
      memoryLimitMiB: 1024,
      taskImageOptions: {
        image: ecs.ContainerImage.fromEcrRepository(repo, "16"),
        environment: {
          PORT: '80',
        },
      },
      publicLoadBalancer: true
    })
    // const cluster = new ecs.Cluster(scope, 'CodeServiceDev', { vpc });
    // cluster.addCapacity('CodeServiceAsgDev', {
    //   instanceType: ec2.InstanceType.of(ec2.InstanceClass.T2, ec2.InstanceSize.MICRO)
    // });
    //
    // const taskDefinition = new ecs.Ec2TaskDefinition(scope, 'TaskDefinisionDev');
    // const repo = ecr.Repository.fromRepositoryName(scope, 'CodeSeriviceRepository', 'code_service');
    // const container = taskDefinition.addContainer('CodeServiceContainerDev', {
    //   image: ecs.ContainerImage.fromEcrRepository(repo, "16"),
    //   memoryLimitMiB: 1024,
    //   environment: {
    //     PORT: '80',
    //   }
    // });
    //
    // container.addPortMappings({
    //   containerPort: 80,
    //   hostPort: 8080,
    //   protocol: ecs.Protocol.TCP,
    // });
    //
    // const service = new ecs.Ec2Service(scope, "CodeServiceServiceDev", {
    //   cluster,
    //   taskDefinition,
    // });
    // const lb = new elbv2.ApplicationLoadBalancer(scope, 'CodeServiceLoadBalancerDev', {
    //   vpc,
    //   internetFacing: true
    // });
    //
    // const listener = lb.addListener('PublicListener', { port: 80, open: true });
    // listener.addTargets('ECS', {
    //   port: 8080,
    //   targets: [service.loadBalancerTarget({
    //     containerName: 'CodeServiceContainerDev',
    //     containerPort: 80
    //   })],
    // });
    //
    // new cdk.CfnOutput(scope, 'LoadBalancerDNS', { value: lb.loadBalancerDnsName, });
  }
}

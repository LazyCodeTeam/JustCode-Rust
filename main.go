package main

import (
	"github.com/LazyCodeTeam/JustCode-Backend/infra"
	aws "github.com/cdktf/cdktf-provider-aws-go/aws/v10/provider"
	"github.com/hashicorp/terraform-cdk-go/cdktf"
)

func main() {
	awsProviderName := "aws"
	awsRegion := "eu-central-1"

	app := cdktf.NewApp(nil)
	aws.NewAwsProvider(app, &awsProviderName, &aws.AwsProviderConfig{
		Region: &awsRegion,
	})

	infra.NewDevStack(app)

	app.Synth()
}

package infra

import (
	"github.com/aws/constructs-go/constructs/v10"
	"github.com/hashicorp/terraform-cdk-go/cdktf"
)

func NewDevStack(scope constructs.Construct) cdktf.TerraformStack {
	stackName := "JustCodeDev"
	stack := cdktf.NewTerraformStack(scope, &stackName)

	// The code that defines your stack goes here

	return stack
}

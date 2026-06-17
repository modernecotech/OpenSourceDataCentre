terraform {
  required_version = ">= 1.8.0"
}

variable "tenant" {
  type = string
}

output "tenant_namespace" {
  value = "tenant-${var.tenant}-dev"
}

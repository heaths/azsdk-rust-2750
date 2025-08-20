// Copyright 2025 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

using './main.bicep'

param environmentName = readEnvironmentVariable('AZURE_ENV_NAME', 'azsdk-rust-2750')
param location = readEnvironmentVariable('AZURE_LOCATION', 'westus')
param principalId = readEnvironmentVariable('AZURE_PRINCIPAL_ID', '')
param vaultName = readEnvironmentVariable('AZURE_KEYVAULT_NAME', '')
param vaultSku = readEnvironmentVariable('AZURE_KEYVAULT_SKU', 'standard')

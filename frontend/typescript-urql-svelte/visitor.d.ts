import { ClientSideBaseVisitor, ClientSideBasePluginConfig, LoadedFragment } from '@graphql-codegen/visitor-plugin-common';
import { UrqlSvelteRawPluginConfig } from './config';
import { OperationDefinitionNode, GraphQLSchema } from 'graphql';
export interface UrqlSveltePluginConfig extends ClientSideBasePluginConfig {
    urqlSvelteImportFrom: string;
}
export declare class UrqlSvelteVisitor extends ClientSideBaseVisitor<UrqlSvelteRawPluginConfig, UrqlSveltePluginConfig> {
    constructor(schema: GraphQLSchema, fragments: LoadedFragment[], rawConfig: UrqlSvelteRawPluginConfig);
    getImports(): string[];
    private _buildHooks;
    protected buildOperation(node: OperationDefinitionNode, documentVariableName: string, operationType: string, operationResultType: string, operationVariablesTypes: string): string;
}

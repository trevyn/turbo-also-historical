import { Types, PluginValidateFn, PluginFunction } from '@graphql-codegen/plugin-helpers';
import { UrqlSvelteVisitor } from './visitor';
import { UrqlSvelteRawPluginConfig } from './config';
export declare const plugin: PluginFunction<UrqlSvelteRawPluginConfig, Types.ComplexPluginOutput>;
export declare const validate: PluginValidateFn<any>;
export { UrqlSvelteVisitor };

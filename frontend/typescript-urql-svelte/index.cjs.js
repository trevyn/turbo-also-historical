'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

function _interopDefault (ex) { return (ex && (typeof ex === 'object') && 'default' in ex) ? ex['default'] : ex; }

const graphql = require('graphql');
const visitorPluginCommon = require('@graphql-codegen/visitor-plugin-common');
const autoBind = _interopDefault(require('auto-bind'));
const path = require('path');

class UrqlSvelteVisitor extends visitorPluginCommon.ClientSideBaseVisitor {
    constructor(schema, fragments, rawConfig) {
        super(schema, fragments, rawConfig, {
            urqlSvelteImportFrom: visitorPluginCommon.getConfigValue(rawConfig.urqlSvelteImportFrom, null),
        });
        autoBind(this);
    }
    getImports() {
        const baseImports = super.getImports();
        const imports = [];
        const hasOperations = this._collectedOperations.length > 0;
        if (!hasOperations) {
            return [`<script context="module" lang="ts">`, ...baseImports];
        }
        imports.push(`import * as UrqlSvelte from '${this.config.urqlSvelteImportFrom || '@urql/svelte'}';`);
        imports.push(visitorPluginCommon.OMIT_TYPE);
        return [
            `import <script context="module" lang="ts">`,
            ...baseImports, ...imports
        ];
    }
    //   private _buildComponent(
    //     node: OperationDefinitionNode,
    //     documentVariableName: string,
    //     operationType: string,
    //     operationResultType: string,
    //     operationVariablesTypes: string
    //   ): string {
    //     const componentNameConverted: string = this.convertName(node.name?.value ?? '', {
    //       suffix: 'Component',
    //       useTypesPrefix: false,
    //     });
    //        const componentName: string = this.convertName(node.name?.value ?? '', {
    //       suffix: 'Component',
    //       useTypesPrefix: false,
    //     });
    //     const isVariablesRequired =
    //       operationType === 'Query' &&
    //       node.variableDefinitions.some(variableDef => variableDef.type.kind === Kind.NON_NULL_TYPE);
    //     const generics = [operationResultType, operationVariablesTypes];
    //     if (operationType === 'Subscription') {
    //       generics.unshift(operationResultType);
    //     }
    //     return `
    // export const ${componentName} = (props: Omit<Urql.${operationType}Props<${generics.join(
    //       ', '
    //     )}>, 'query'> & { variables${isVariablesRequired ? '' : '?'}: ${operationVariablesTypes} }) => (
    //   <Urql.${operationType} {...props} query={${documentVariableName}} />
    // );
    // `;
    //   }
    _buildHooks(node, operationType, documentVariableName, operationResultType, operationVariablesTypes) {
        var _a, _b;
        const operationNameConverted = this.convertName((_b = (_a = node.name) === null || _a === void 0 ? void 0 : _a.value) !== null && _b !== void 0 ? _b : '', {
            suffix: this.config.omitOperationSuffix ? '' : operationType,
            useTypesPrefix: false,
        });
        const operationName = operationNameConverted.slice(0, 1).toLowerCase() + operationNameConverted.slice(1);
        //     if (operationType === 'Mutation') {
        //       return `
        // export function ${operationName}() {
        //   return UrqlSvelte.use${operationType}<${operationResultType}, ${operationVariablesTypes}>(${documentVariableName});
        // };`;
        //     }
        //     if (operationType === 'Subscription') {
        //       return `
        // export function ${operationName}<TData = ${operationResultType}>(options: Omit<Urql.Use${operationType}Args<${operationVariablesTypes}>, 'query'> = {}, handler?: Urql.SubscriptionHandler<${operationResultType}, TData>) {
        //   return Urql.use${operationType}<${operationResultType}, TData, ${operationVariablesTypes}>({ query: ${documentVariableName}, ...options }, handler);
        // };`;
        //     }
        if (operationType === 'Subscription') {
            return `
export function ${operationName}(handler) {
  return UrqlSvelte.subscription(UrqlSvelte.operationStore(${documentVariableName}), handler);
};`;
        }
        else if (operationType === 'Mutation') {
            return `
export function ${operationName}() {
  return UrqlSvelte.mutation(UrqlSvelte.operationStore(${documentVariableName}));
};`;
        }
        else {
            return `
export function ${operationName}() {
  return UrqlSvelte.query(UrqlSvelte.operationStore(${documentVariableName}));
};`;
        }
    }
    // query(operationStore(`
    //    query {
    //      listPdfs {
    //        rowid
    //        name
    //      }
    //    }
    //  `));
    //      if (operationType === 'Mutation') {
    //       return `
    // export function use${operationName}() {
    //   return Urql.use${operationType}<${operationResultType}, ${operationVariablesTypes}>(${documentVariableName});
    // };`;
    //     }
    //     if (operationType === 'Subscription') {
    //       return `
    // export function use${operationName}<TData = ${operationResultType}>(options: Omit<Urql.Use${operationType}Args<${operationVariablesTypes}>, 'query'> = {}, handler?: Urql.SubscriptionHandler<${operationResultType}, TData>) {
    //   return Urql.use${operationType}<${operationResultType}, TData, ${operationVariablesTypes}>({ query: ${documentVariableName}, ...options }, handler);
    // };`;
    //     }
    //     return `
    // export function use${operationName}(options: Omit<Urql.Use${operationType}Args<${operationVariablesTypes}>, 'query'> = {}) {
    //   return Urql.use${operationType}<${operationResultType}>({ query: ${documentVariableName}, ...options });
    // };`;
    //   }
    buildOperation(node, documentVariableName, operationType, operationResultType, operationVariablesTypes) {
        return this._buildHooks(node, operationType, documentVariableName, operationResultType, operationVariablesTypes);
        // return [component, hooks].filter(a => a).join('\n');
    }
}

const plugin = (schema, documents, config) => {
    const allAst = graphql.concatAST(documents.map(v => v.document));
    const allFragments = [
        ...allAst.definitions.filter(d => d.kind === graphql.Kind.FRAGMENT_DEFINITION).map(fragmentDef => ({
            node: fragmentDef,
            name: fragmentDef.name.value,
            onType: fragmentDef.typeCondition.name.value,
            isExternal: false,
        })),
        ...(config.externalFragments || []),
    ];
    const visitor = new UrqlSvelteVisitor(schema, allFragments, config);
    const visitorResult = graphql.visit(allAst, { leave: visitor });
    return {
        prepend: visitor.getImports(),
        content: [visitor.fragments, ...visitorResult.definitions.filter(t => typeof t === 'string'), `</script>`].join('\n'),
    };
};
const validate = async (schema, documents, config, outputFile) => {
    if (path.extname(outputFile) !== '.svelte') {
        throw new Error(`Plugin "typescript-urql-svelte" requires extension to be ".svelte"!`);
    }
};

exports.UrqlSvelteVisitor = UrqlSvelteVisitor;
exports.plugin = plugin;
exports.validate = validate;
//# sourceMappingURL=index.cjs.js.map

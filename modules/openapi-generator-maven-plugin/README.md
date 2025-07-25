openapi-generator-maven-plugin
============================

A Maven plugin to support the OpenAPI generator project

Usage
============================

Add to your `build->plugins` section (default phase is `generate-sources` phase)
```xml
<plugin>
    <groupId>org.openapitools</groupId>
    <artifactId>openapi-generator-maven-plugin</artifactId>
    <!-- RELEASE_VERSION -->
    <version>7.14.0</version>
    <!-- /RELEASE_VERSION -->
    <executions>
        <execution>
            <goals>
                <goal>generate</goal>
            </goals>
            <configuration>
                <inputSpec>${project.basedir}/src/main/resources/api.yaml</inputSpec>
                <generatorName>java</generatorName>
                <configOptions>
                   <sourceFolder>src/gen/java/main</sourceFolder>
                </configOptions>
            </configuration>
        </execution>
    </executions>
</plugin>
```

Followed by:

```
mvn clean compile
```

### General Configuration parameters

:bulb: These **general** configurations should be in the same level

| Option | Property | Description |
|--------|----------|-------------|
| `verbose` |  `openapi.generator.maven.plugin.verbose` | verbose mode (`false` by default)
| `inputSpec` |  `openapi.generator.maven.plugin.inputSpec` | OpenAPI Spec file path
| `inputSpecRootDirectory` |  `openapi.generator.maven.plugin.inputSpecRootDirectory` | Local root folder with spec file(s)
| `mergedFileName` |  `openapi.generator.maven.plugin.mergedFileName` | Name of the file that will contain all merged specs
| `language` |  `openapi.generator.maven.plugin.language` | target generation language (deprecated, replaced by `generatorName` as values here don't represent only 'language' any longer)
| `generatorName` |  `openapi.generator.maven.plugin.generatorName` | target generator name
| `cleanupOutput` |  `openapi.generator.maven.plugin.cleanupOutput` | Defines whether the output directory should be cleaned up before generating the output (`false` by default).
| `output` |  `openapi.generator.maven.plugin.output` | target output path (default is `${project.build.directory}/generated-sources/openapi`. Can also be set globally through the `openapi.generator.maven.plugin.output` property)
| `gitHost` | `openapi.generator.maven.plugin.gitHost` | The git host, e.g. gitlab.com
| `gitUserId` |  `openapi.generator.maven.plugin.gitUserId` | sets git information of the project
| `gitRepoId` | `openapi.generator.maven.plugin.gitRepoId` | sets the repo ID (e.g. openapi-generator)
| `collapsedSpec` | `openapi.generator.maven.plugin.collapsedSpec` | sets the path to the collapsed single-file representation of the OpenAPI spec
| `includeCollapsedSpecInArtifacts` | `openapi.generator.maven.plugin.publishCollapsedSpec` | includes the collapsed spec in the Maven artifacts (`false` by default)
| `templateDirectory` |  `openapi.generator.maven.plugin.templateDirectory` | directory with mustache templates
| `templateResourcePath` |  `openapi.generator.maven.plugin.templateResourcePath` | directory with mustache templates via resource path. This option will overwrite any option defined in `templateDirectory`.
| `engine` | `openapi.generator.maven.plugin.engine` | The name of templating engine to use, "mustache" (default) or "handlebars" (beta)
| `auth` |  `openapi.generator.maven.plugin.auth` | adds authorization headers when fetching the OpenAPI definitions remotely. Pass in a URL-encoded string of `name:header` with a comma separating multiple values
| `configurationFile` |  `openapi.generator.maven.plugin.configurationFile` | Path to separate json configuration file. File content should be in a json format {"optionKey":"optionValue", "optionKey1":"optionValue1"...} Supported options can be different for each generator. Run `config-help -g {generator name}` command for generator-specific config options
| `skipOverwrite` |  `openapi.generator.maven.plugin.skipOverwrite` | Specifies if the existing files should be overwritten during the generation. (`false` by default)
| `apiPackage` |  `openapi.generator.maven.plugin.apiPackage` | the package to use for generated api objects/classes
| `modelPackage` |  `openapi.generator.maven.plugin.modelPackage` | the package to use for generated model objects/classes
| `invokerPackage` |  `openapi.generator.maven.plugin.invokerPackage` | the package to use for the generated invoker objects
| `packageName` | `openapi.generator.maven.plugin.packageName` | the default package name to use for the generated objects
| `groupId` | `openapi.generator.maven.plugin.groupId`  | sets project information in generated pom.xml/build.gradle or other build script. Language-specific conversions occur in non-jvm generators
| `artifactId` |  `openapi.generator.maven.plugin.artifactId` | sets project information in generated pom.xml/build.gradle or other build script. Language-specific conversions occur in non-jvm generators
| `artifactVersion` |  `openapi.generator.maven.plugin.artifactVersion` | sets project information in generated pom.xml/build.gradle or other build script. Language-specific conversions occur in non-jvm generators
| `library` |  `openapi.generator.maven.plugin.library` | library template (sub-template)
| `modelNamePrefix` |  `openapi.generator.maven.plugin.modelNamePrefix` | Sets the prefix for model classes and enums
| `modelNameSuffix` |  `openapi.generator.maven.plugin.modelNameSuffix` | Sets the suffix for model classes and enums
| `apiNameSuffix` |  `openapi.generator.maven.plugin.apiNameSuffix` | Sets the suffix for api classes
| `ignoreFileOverride` |  `openapi.generator.maven.plugin.ignoreFileOverride` | specifies the full path to a `.openapi-generator-ignore` used for pattern based overrides of generated outputs
| `httpUserAgent` | `openapi.generator.maven.plugin.httpUserAgent` | Sets custom User-Agent header value
| `removeOperationIdPrefix` |  `openapi.generator.maven.plugin.removeOperationIdPrefix` | remove operationId prefix (e.g. user_getName => getName)
| `skipOperationExample` |  `openapi.generator.maven.plugin.skipOperationExample` | skip examples defined in the operation
| `logToStderr` |  `openapi.generator.maven.plugin.logToStderr` | write all log messages (not just errors) to STDERR
| `enablePostProcessFile` |  `openapi.generator.maven.plugin.` | enable file post-processing hook
| `skipValidateSpec` |  `openapi.generator.maven.plugin.skipValidateSpec` | Whether or not to skip validating the input spec prior to generation. By default, invalid specifications will result in an error.
| `strictSpec` |  `openapi.generator.maven.plugin.strictSpec` | Whether or not to treat an input document strictly against the spec. 'MUST' and 'SHALL' wording in OpenAPI spec is strictly adhered to. e.g. when false, no fixes will be applied to documents which pass validation but don't follow the spec.
| `openapiNormalizer` |  `openapi.generator.maven.plugin.openapiNormalizer` | specifies the rules to be enabled in OpenAPI normalizer in the form of RULE_1=true,RULE_2=original.
| `generateAliasAsModel` |  `openapi.generator.maven.plugin.generateAliasAsModel` | generate alias (array, map) as model
| `configOptions` |  N/A | a **map** of generator-specific parameters. To show a full list of generator-specified parameters (options), please use `configHelp` (explained below)
| `instantiationTypes` |  `openapi.generator.maven.plugin.instantiationTypes` | sets instantiation type mappings in the format of type=instantiatedType,type=instantiatedType. For example (in Java): `array=ArrayList,map=HashMap`. In other words array types will get instantiated as ArrayList in generated code. You can also have multiple occurrences of this option
| `importMappings` |  `openapi.generator.maven.plugin.importMappings` | specifies mappings between a given class and the import that should be used for that class in the format of type=import,type=import. You can also have multiple occurrences of this option
| `typeMappings` |  `openapi.generator.maven.plugin.typeMappings` | sets mappings between OpenAPI spec types and generated code types in the format of OpenAPIType=generatedType,OpenAPIType=generatedType. For example: `array=List,map=Map,string=String`. You can also have multiple occurrences of this option. To map a specified format, use type+format, e.g. string+password=EncryptedString will map `type: string, format: password` to `EncryptedString`.
| `schemaMappings` |  `openapi.generator.maven.plugin.schemaMappings` | specifies mappings between the schema and the new name in the format of schema_a=Cat,schema_b=Bird. https://openapi-generator.tech/docs/customization/#schema-mapping
| `nameMappings` |  `openapi.generator.maven.plugin.nameMappings` | specifies mappings between the property name and the new name in the format of property_a=firstProperty,property_b=secondProperty. https://openapi-generator.tech/docs/customization/#name-mapping
| `modelNameMappings` |  `openapi.generator.maven.plugin.modelNameMappings` | specifies mappings between the model name and the new name in the format of model_a=FirstModel,model_b=SecondModel. https://openapi-generator.tech/docs/customization/#name-mapping
| `parameterNameMappings` |  `openapi.generator.maven.plugin.parameterNameMappings` | specifies mappings between the parameter name and the new name in the format of param_a=first_parameter,param_b=second_parameter. https://openapi-generator.tech/docs/customization/#name-mapping
| `inlineSchemaNameMappings` |  `openapi.generator.maven.plugin.inlineSchemaNameMappings` | specifies mappings between the inline schema name and the new name in the format of inline_object_2=Cat,inline_object_5=Bird.
| `inlineSchemaOptions` |  `openapi.generator.maven.plugin.inlineSchemaOptions` | specifies the options used when naming inline schema in inline model resolver
| `languageSpecificPrimitives` |  `openapi.generator.maven.plugin.languageSpecificPrimitives` | specifies additional language specific primitive types in the format of type1,type2,type3,type3. For example: `String,boolean,Boolean,Double`. You can also have multiple occurrences of this option
| `additionalProperties` |  `openapi.generator.maven.plugin.additionalProperties` | sets additional properties that can be referenced by the mustache templates in the format of name=value,name=value. You can also have multiple occurrences of this option
| `serverVariableOverrides` | `openapi.generator.maven.plugin.serverVariableOverrides` | A map of server variable overrides for specs that support server URL templating
| `reservedWordsMappings` |  `openapi.generator.maven.plugin.reservedWordsMappings` | specifies how a reserved name should be escaped to. Otherwise, the default `_<name>` is used. For example `id=identifier`. You can also have multiple occurrences of this option
| `generateApis` |  `openapi.generator.maven.plugin.generateApis` | generate the apis (`true` by default). Specific apis may be defined as a CSV via `apisToGenerate`.
| `apisToGenerate` |  `openapi.generator.maven.plugin.apisToGenerate` | A comma separated list of apis to generate.  All apis is the default.
| `generateModels` |  `openapi.generator.maven.plugin.generateModels` | generate the models (`true` by default). Specific models may be defined as a CSV via `modelsToGenerate`.
| `modelsToGenerate` |  `openapi.generator.maven.plugin.modelsToGenerate` | A comma separated list of models to generate.  All models is the default.
| `generateSupportingFiles` |  `openapi.generator.maven.plugin.generateSupportingFiles` | generate the supporting files (`true` by default)
| `supportingFilesToGenerate` |  `openapi.generator.maven.plugin.supportingFilesToGenerate` | A comma separated list of supporting files to generate.  All files is the default.
| `generateModelTests` |  `openapi.generator.maven.plugin.generateModelTests` | generate the model tests (`true` by default. Only available if `generateModels` is `true`)
| `generateModelDocumentation` |  `openapi.generator.maven.plugin.generateModelDocumentation` | generate the model documentation (`true` by default. Only available if `generateModels` is `true`)
| `generateApiTests` |  `openapi.generator.maven.plugin.generateApiTests` | generate the api tests (`true` by default. Only available if `generateApis` is `true`)
| `generateApiDocumentation` |  `openapi.generator.maven.plugin.generateApiDocumentation` | generate the api documentation (`true` by default. Only available if `generateApis` is `true`)
| `skip` |  `codegen.skip` | skip code generation (`false` by default. Can also be set globally through the `codegen.skip` property)
| `skipIfSpecIsUnchanged` |  `codegen.skipIfSpecIsUnchanged` | Skip the execution if the source file is older than the output folder (`false` by default. Can also be set globally through the `codegen.skipIfSpecIsUnchanged` property)
| `addCompileSourceRoot` |  `openapi.generator.maven.plugin.addCompileSourceRoot` | Add the output directory to the project as a source root, so that the generated java types are compiled and included in the project artifact (`true` by default). Mutually exclusive with `addTestCompileSourceRoot`.
| `addTestCompileSourceRoot` |  `openapi.generator.maven.plugin.addTestCompileSourceRoot` | Add the output directory to the project as a test source root, so that the generated java types are compiled only for the test classpath of the project (`false` by default). Mutually exclusive with `addCompileSourceRoot`.
| `dryRun` | `openapi.generator.maven.plugin.dryRun` | Defines whether the generator should run in dry-run mode. In dry-run mode no files are written and a summary about file states is output ( `false` by default).
| `environmentVariables` | N/A | deprecated. Use globalProperties instead.
| `globalProperties` | N/A | A **map** of items conceptually similar to "environment variables" or "system properties". These are available to all aspects of the generation flow. See [Global Properties](https://openapi-generator.tech/docs/globals/) for list of available properties.
| `configHelp` |  `codegen.configHelp` | dumps the configuration help for the specified library (generates no sources)

### Configuring **map** structures

For configuration options documented as a **map** above, the key/value options may be configured as free-form nodes under these options. This takes the format:

```xml
<configuration>
    <option>
       <key>value</key>
    </option>
</configuration>
```

This configuration node location will match that of the plugin configuration examples at the top of this document and in the section below. Here, `option` matches in option name in the first column in the table from the previous section.
The `key` and `value` text are any values you'd like to provide for that option. As an example, to configure `globalProperties` to match the `--global-property models=User:Pet` example from our [Selective Generation](https://openapi-generator.tech/docs/customization#selective-generation) documentation, see below.

```xml
<configuration>
    <globalProperties>
       <models>User:Pet</models>
    </globalProperties>
</configuration>
```

Notice that some of these environment variable options may overwrite or conflict with other options available to the maven plugin. For example, the above `globalProperties` example is equivalent to the following:

```xml
<configuration>
    <generateModels>true</generateModels>
    <modelsToGenerate>User:Pet</modelsToGenerate>
</configuration>
```

The difference here is that you may define `generateModels` and `modelsToGenerate` as properties, while `globalProperties` may only be configured as a configuration node.

### Type and import mappings

To override the mappings between OpenAPI spec types and the types used in the generated code, set `typeMappings`.

```xml
<configuration>
    <typeMappings>
        <!-- convert Double to BigDecimal -->
        <typeMapping>Double=java.math.BigDecimal</typeMapping>
    </typeMappings>
</configuration>
```

For types that are not already included in the generator configuration, you may need to add a corresponding `importMapping` too.

```xml
<configuration>
    <!-- convert file/binary to JAX-RS StreamingOutput -->
    <typeMappings>
        <typeMapping>binary=StreamingOutput</typeMapping>
        <typeMapping>file=StreamingOutput</typeMapping>
    </typeMappings>
    <importMappings>
        <importMapping>StreamingOutput=javax.ws.rs.core.StreamingOutput</importMapping>
    </importMappings>
</configuration>
```


### Custom Generator

Specifying a custom generator is a bit different. It doesn't support the classpath:/ syntax, but it does support the fully qualified name of the package. You can also specify your custom templates, which also get pulled in. Notice the dependency on a project, in the plugin scope. That would be your generator/template jar.

```xml
<plugin>
    <groupId>org.openapitools</groupId>
    <artifactId>openapi-generator-maven-plugin</artifactId>
    <version>${openapi-generator-maven-plugin-version}</version>
    <executions>
        <execution>
            <goals>
                <goal>generate</goal>
            </goals>
            <configuration>
                <inputSpec>${project.basedir}/src/main/resources/yaml/yamlfilename.yaml</inputSpec>
                <!-- language file, like e.g. JavaJaxRSCodegen -->
                <generatorName>com.my.package.for.GeneratorLanguage</generatorName>
                <templateDirectory>myTemplateDir</templateDirectory>

                <output>${project.build.directory}/generated-sources</output>
                <apiPackage>${default.package}.handler</apiPackage>
                <modelPackage>${default.package}.model</modelPackage>
                <invokerPackage>${default.package}.handler</invokerPackage>
            </configuration>
        </execution>
    </executions>

    <dependencies>
        <dependency>
            <groupId>com.my.generator</groupId>
            <artifactId>customgenerator</artifactId>
            <version>1.0-SNAPSHOT</version>
        </dependency>
    </dependencies>
</plugin>
```

### Sample configuration

Please see [an example configuration](examples) for using the plugin. To run these examples, explicitly pass the file to maven. Example:

```bash
mvn -f non-java.xml compile
```

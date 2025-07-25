package org.openapitools.api;

import java.math.BigDecimal;
import org.openapitools.model.ChildWithNullable;
import org.openapitools.model.Client;
import java.util.Date;
import org.openapitools.model.EnumClass;
import org.openapitools.model.FakeBigDecimalMap200Response;
import org.openapitools.model.FakeTestsDefaultsDefaultResponse;
import java.io.File;
import org.openapitools.model.FileSchemaTestClass;
import org.openapitools.model.HealthCheckResult;
import org.joda.time.LocalDate;
import java.util.Map;
import org.openapitools.model.ModelApiResponse;
import org.openapitools.model.OuterComposite;
import org.openapitools.model.OuterObjectWithEnumProperty;
import org.openapitools.model.Pet;
import org.openapitools.model.TestInlineFreeformAdditionalPropertiesRequest;
import org.openapitools.model.User;

import javax.ws.rs.*;
import javax.ws.rs.core.Response;

import io.swagger.annotations.*;

import java.io.InputStream;
import java.util.Map;
import java.util.List;
import javax.validation.constraints.*;
import javax.validation.Valid;

/**
* Represents a collection of functions to interact with the API endpoints.
*/
@Path("/fake")
@Api(description = "the fake API")
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaJAXRSSpecServerCodegen", comments = "Generator version: 7.15.0-SNAPSHOT")
public class FakeApi {

    @GET
    @Path("/BigDecimalMap")
    @Produces({ "*/*" })
    @ApiOperation(value = "", notes = "for Java apache and Java native, test toUrlQueryString for maps with BegDecimal keys", response = FakeBigDecimalMap200Response.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = FakeBigDecimalMap200Response.class)
    })
    public Response fakeBigDecimalMap() {
        return Response.ok().entity("magic!").build();
    }

    @GET
    @Path("/health")
    @Produces({ "application/json" })
    @ApiOperation(value = "Health check endpoint", notes = "", response = HealthCheckResult.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "The instance started successfully", response = HealthCheckResult.class)
    })
    public Response fakeHealthGet() {
        return Response.ok().entity("magic!").build();
    }

    @GET
    @Path("/http-signature-test")
    @Consumes({ "application/json", "application/xml" })
    @ApiOperation(value = "test http signature authentication", notes = "", response = Void.class, authorizations = {
        
        @Authorization(value = "http_signature_test")
         }, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "The instance started successfully", response = Void.class)
    })
    public Response fakeHttpSignatureTest(@Valid @NotNull Pet pet,@QueryParam("query_1")  @ApiParam("query parameter")  String query1,@HeaderParam("header_1")   @ApiParam("header parameter") String header1) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/outer/boolean")
    @Consumes({ "application/json" })
    @Produces({ "*/*" })
    @ApiOperation(value = "", notes = "Test serialization of outer boolean types", response = Boolean.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Output boolean", response = Boolean.class)
    })
    public Response fakeOuterBooleanSerialize(@Valid Boolean body) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/outer/composite")
    @Consumes({ "application/json" })
    @Produces({ "*/*" })
    @ApiOperation(value = "", notes = "Test serialization of object with outer number type", response = OuterComposite.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Output composite", response = OuterComposite.class)
    })
    public Response fakeOuterCompositeSerialize(@Valid OuterComposite outerComposite) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/outer/number")
    @Consumes({ "application/json" })
    @Produces({ "*/*" })
    @ApiOperation(value = "", notes = "Test serialization of outer number types", response = BigDecimal.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Output number", response = BigDecimal.class)
    })
    public Response fakeOuterNumberSerialize(@Valid BigDecimal body) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/outer/string")
    @Consumes({ "application/json" })
    @Produces({ "*/*" })
    @ApiOperation(value = "", notes = "Test serialization of outer string types", response = String.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Output string", response = String.class)
    })
    public Response fakeOuterStringSerialize(@Valid String body) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/property/enum-int")
    @Consumes({ "application/json" })
    @Produces({ "*/*" })
    @ApiOperation(value = "", notes = "Test serialization of enum (int) properties with examples", response = OuterObjectWithEnumProperty.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Output enum (int)", response = OuterObjectWithEnumProperty.class)
    })
    public Response fakePropertyEnumIntegerSerialize(@Valid @NotNull OuterObjectWithEnumProperty outerObjectWithEnumProperty) {
        return Response.ok().entity("magic!").build();
    }

    @GET
    @Path("/tests/defaults")
    @Produces({ "application/json" })
    @ApiOperation(value = "test enum default in request body", notes = "", response = FakeTestsDefaultsDefaultResponse.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "response", response = FakeTestsDefaultsDefaultResponse.class)
    })
    public Response fakeTestsDefaults() {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/additionalProperties-reference")
    @Consumes({ "application/json" })
    @ApiOperation(value = "test referenced additionalProperties", notes = "", response = Void.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class)
    })
    public Response testAdditionalPropertiesReference(@Valid @NotNull Map<String, Object> requestBody) {
        return Response.ok().entity("magic!").build();
    }

    @PUT
    @Path("/body-with-binary")
    @Consumes({ "image/png" })
    @ApiOperation(value = "", notes = "For this test, the body has to be a binary file.", response = Void.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Success", response = Void.class)
    })
    public Response testBodyWithBinary(@Valid File body) {
        return Response.ok().entity("magic!").build();
    }

    @PUT
    @Path("/body-with-file-schema")
    @Consumes({ "application/json" })
    @ApiOperation(value = "", notes = "For this test, the body for this request must reference a schema named `File`.", response = Void.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Success", response = Void.class)
    })
    public Response testBodyWithFileSchema(@Valid @NotNull FileSchemaTestClass fileSchemaTestClass) {
        return Response.ok().entity("magic!").build();
    }

    @PUT
    @Path("/body-with-query-params")
    @Consumes({ "application/json" })
    @ApiOperation(value = "", notes = "", response = Void.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Success", response = Void.class)
    })
    public Response testBodyWithQueryParams(@QueryParam("query") @NotNull   String query,@Valid @NotNull User user) {
        return Response.ok().entity("magic!").build();
    }

    @PATCH
    @Consumes({ "application/json" })
    @Produces({ "application/json" })
    @ApiOperation(value = "To test \"client\" model", notes = "To test \"client\" model", response = Client.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Client.class)
    })
    public Response testClientModel(@Valid @NotNull Client client) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Consumes({ "application/x-www-form-urlencoded" })
    @ApiOperation(value = "Fake endpoint for testing various parameters 假端點 偽のエンドポイント 가짜 엔드 포인트 ", notes = "Fake endpoint for testing various parameters 假端點 偽のエンドポイント 가짜 엔드 포인트 ", response = Void.class, authorizations = {
        
        @Authorization(value = "http_basic_test")
         }, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 400, message = "Invalid username supplied", response = Void.class),
        @ApiResponse(code = 404, message = "User not found", response = Void.class)
    })
    public Response testEndpointParameters(@FormParam(value = "number")  BigDecimal number,@FormParam(value = "double")  Double _double,@FormParam(value = "pattern_without_delimiter")  String patternWithoutDelimiter,@FormParam(value = "byte")  byte[] _byte,@FormParam(value = "integer")  Integer integer,@FormParam(value = "int32")  Integer int32,@FormParam(value = "int64")  Long int64,@FormParam(value = "float")  Float _float,@FormParam(value = "string")  String string, @FormParam(value = "binary") InputStream binaryInputStream,@FormParam(value = "date")  LocalDate date,@FormParam(value = "dateTime")  Date dateTime,@FormParam(value = "password")  String password,@FormParam(value = "callback")  String paramCallback) {
        return Response.ok().entity("magic!").build();
    }

    @GET
    @Consumes({ "application/x-www-form-urlencoded" })
    @ApiOperation(value = "To test enum parameters", notes = "To test enum parameters", response = Void.class, tags={ "fake" })
    @io.swagger.annotations.ApiImplicitParams({
        @io.swagger.annotations.ApiImplicitParam(name = "enum_header_string", value = "Header parameter enum test (string)",  dataType = "String", paramType = "header")
    })
    @ApiResponses(value = { 
        @ApiResponse(code = 400, message = "Invalid request", response = Void.class),
        @ApiResponse(code = 404, message = "Not found", response = Void.class)
    })
    public Response testEnumParameters(@HeaderParam("enum_header_string_array")   @ApiParam("Header parameter enum test (string array)") List<String> enumHeaderStringArray,@QueryParam("enum_query_string_array")  @ApiParam("Query parameter enum test (string array)")  List<String> enumQueryStringArray,@QueryParam("enum_query_string") @DefaultValue("-efg")  @ApiParam("Query parameter enum test (string)")  String enumQueryString,@QueryParam("enum_query_integer")  @ApiParam("Query parameter enum test (double)")  Integer enumQueryInteger,@QueryParam("enum_query_double")  @ApiParam("Query parameter enum test (double)")  Double enumQueryDouble,@QueryParam("enum_query_model_array")   List<EnumClass> enumQueryModelArray,@FormParam(value = "enum_form_string_array")  List<String> enumFormStringArray,@FormParam(value = "enum_form_string")  String enumFormString) {
        return Response.ok().entity("magic!").build();
    }

    @DELETE
    @ApiOperation(value = "Fake endpoint to test group parameters (optional)", notes = "Fake endpoint to test group parameters (optional)", response = Void.class, authorizations = {
        
        @Authorization(value = "bearer_test")
         }, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 400, message = "Something wrong", response = Void.class)
    })
    public Response testGroupParameters(@QueryParam("required_string_group") @NotNull  @ApiParam("Required String in group parameters")  Integer requiredStringGroup,@HeaderParam("required_boolean_group") @NotNull   @ApiParam("Required Boolean in group parameters") Boolean requiredBooleanGroup,@QueryParam("required_int64_group") @NotNull  @ApiParam("Required Integer in group parameters")  Long requiredInt64Group,@QueryParam("string_group")  @ApiParam("String in group parameters")  Integer stringGroup,@HeaderParam("boolean_group")   @ApiParam("Boolean in group parameters") Boolean booleanGroup,@QueryParam("int64_group")  @ApiParam("Integer in group parameters")  Long int64Group) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/inline-additionalProperties")
    @Consumes({ "application/json" })
    @ApiOperation(value = "test inline additionalProperties", notes = "", response = Void.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class)
    })
    public Response testInlineAdditionalProperties(@Valid @NotNull Map<String, String> requestBody) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/inline-freeform-additionalProperties")
    @Consumes({ "application/json" })
    @ApiOperation(value = "test inline free-form additionalProperties", notes = "", response = Void.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class)
    })
    public Response testInlineFreeformAdditionalProperties(@Valid @NotNull TestInlineFreeformAdditionalPropertiesRequest testInlineFreeformAdditionalPropertiesRequest) {
        return Response.ok().entity("magic!").build();
    }

    @GET
    @Path("/jsonFormData")
    @Consumes({ "application/x-www-form-urlencoded" })
    @ApiOperation(value = "test json serialization of form data", notes = "", response = Void.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class)
    })
    public Response testJsonFormData(@FormParam(value = "param")  String param,@FormParam(value = "param2")  String param2) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/nullable")
    @Consumes({ "application/json" })
    @ApiOperation(value = "test nullable parent property", notes = "", response = Void.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class)
    })
    public Response testNullable(@Valid @NotNull ChildWithNullable childWithNullable) {
        return Response.ok().entity("magic!").build();
    }

    @PUT
    @Path("/test-query-parameters")
    @ApiOperation(value = "", notes = "To test the collection format in query parameters", response = Void.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "Success", response = Void.class)
    })
    public Response testQueryParameterCollectionFormat(@QueryParam("pipe") @NotNull   List<String> pipe,@QueryParam("ioutil") @NotNull   List<String> ioutil,@QueryParam("http") @NotNull   List<String> http,@QueryParam("url") @NotNull   List<String> url,@QueryParam("context") @NotNull   List<String> context,@QueryParam("allowEmpty") @NotNull   String allowEmpty,@QueryParam("language")   Map<String, String> language) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/stringMap-reference")
    @Consumes({ "application/json" })
    @ApiOperation(value = "test referenced string map", notes = "", response = Void.class, tags={ "fake" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class)
    })
    public Response testStringMapReference(@Valid @NotNull Map<String, String> requestBody) {
        return Response.ok().entity("magic!").build();
    }

    @POST
    @Path("/{petId}/uploadImageWithRequiredFile")
    @Consumes({ "multipart/form-data" })
    @Produces({ "application/json" })
    @ApiOperation(value = "uploads an image (required)", notes = "", response = ModelApiResponse.class, authorizations = {
        @Authorization(value = "petstore_auth", scopes = {
            @AuthorizationScope(scope = "write:pets", description = "modify pets in your account"),
            @AuthorizationScope(scope = "read:pets", description = "read your pets") })
         }, tags={ "pet" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = ModelApiResponse.class)
    })
    public Response uploadFileWithRequiredFile(@PathParam("petId") @ApiParam("ID of pet to update") Long petId, @FormParam(value = "requiredFile") InputStream requiredFileInputStream,@FormParam(value = "additionalMetadata")  String additionalMetadata) {
        return Response.ok().entity("magic!").build();
    }
}

package org.openapitools.api;

import java.util.Map;
import org.openapitools.model.Order;

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
@Path("/store")
@Api(description = "the store API")
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaJAXRSSpecServerCodegen", comments = "Generator version: 7.15.0-SNAPSHOT")
public interface StoreApi {

    /**
     * For valid response try integer IDs with value < 1000. Anything above 1000 or nonintegers will generate API errors
     *
     * @param orderId ID of the order that needs to be deleted
     * @return Invalid ID supplied
     * @return Order not found
     */
    @DELETE
    @Path("/order/{order_id}")
    @ApiOperation(value = "Delete purchase order by ID", notes = "For valid response try integer IDs with value < 1000. Anything above 1000 or nonintegers will generate API errors", tags={ "store" })
    @ApiResponses(value = { 
        @ApiResponse(code = 400, message = "Invalid ID supplied", response = Void.class),
        @ApiResponse(code = 404, message = "Order not found", response = Void.class) })
    Response deleteOrder(@PathParam("order_id") @ApiParam("ID of the order that needs to be deleted") String orderId);


    /**
     * Returns a map of status codes to quantities
     *
     * @return successful operation
     */
    @GET
    @Path("/inventory")
    @Produces({ "application/json" })
    @ApiOperation(value = "Returns pet inventories by status", notes = "Returns a map of status codes to quantities", authorizations = {
        
        @Authorization(value = "api_key")
         }, tags={ "store" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Map.class, responseContainer = "Map") })
    Response getInventory();


    /**
     * For valid response try integer IDs with value <= 5 or > 10. Other values will generate exceptions
     *
     * @param orderId ID of pet that needs to be fetched
     * @return successful operation
     * @return Invalid ID supplied
     * @return Order not found
     */
    @GET
    @Path("/order/{order_id}")
    @Produces({ "application/xml", "application/json" })
    @ApiOperation(value = "Find purchase order by ID", notes = "For valid response try integer IDs with value <= 5 or > 10. Other values will generate exceptions", tags={ "store" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Order.class),
        @ApiResponse(code = 400, message = "Invalid ID supplied", response = Void.class),
        @ApiResponse(code = 404, message = "Order not found", response = Void.class) })
    Response getOrderById(@PathParam("order_id") @Min(1L) @Max(5L) @ApiParam("ID of pet that needs to be fetched") Long orderId);


    /**
     * 
     *
     * @param body order placed for purchasing the pet
     * @return successful operation
     * @return Invalid Order
     */
    @POST
    @Path("/order")
    @Produces({ "application/xml", "application/json" })
    @ApiOperation(value = "Place an order for a pet", notes = "", tags={ "store" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Order.class),
        @ApiResponse(code = 400, message = "Invalid Order", response = Void.class) })
    Response placeOrder(@Valid @NotNull Order body);

}

/*
 * OpenAPI Petstore
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package org.openapitools.client.model;

import java.util.Objects;
import java.util.Arrays;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonTypeName;
import com.fasterxml.jackson.annotation.JsonValue;
import java.time.OffsetDateTime;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;
import com.fasterxml.jackson.annotation.JsonTypeName;

/**
 * An order for a pets from the pet store
 */
@JsonPropertyOrder({
  Order.JSON_PROPERTY_ID,
  Order.JSON_PROPERTY_PET_ID,
  Order.JSON_PROPERTY_QUANTITY,
  Order.JSON_PROPERTY_SHIP_DATE,
  Order.JSON_PROPERTY_STATUS,
  Order.JSON_PROPERTY_COMPLETE
})
@jakarta.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.15.0-SNAPSHOT")
public class Order {
  public static final String JSON_PROPERTY_ID = "id";
  @jakarta.annotation.Nullable
  private Long id;

  public static final String JSON_PROPERTY_PET_ID = "petId";
  @jakarta.annotation.Nullable
  private Long petId;

  public static final String JSON_PROPERTY_QUANTITY = "quantity";
  @jakarta.annotation.Nullable
  private Integer quantity;

  public static final String JSON_PROPERTY_SHIP_DATE = "shipDate";
  @jakarta.annotation.Nullable
  private OffsetDateTime shipDate;

  /**
   * Order Status
   */
  public enum StatusEnum {
    PLACED(String.valueOf("placed")),
    
    APPROVED(String.valueOf("approved")),
    
    DELIVERED(String.valueOf("delivered"));

    private String value;

    StatusEnum(String value) {
      this.value = value;
    }

    @JsonValue
    public String getValue() {
      return value;
    }

    @Override
    public String toString() {
      return String.valueOf(value);
    }

    @JsonCreator
    public static StatusEnum fromValue(String value) {
      for (StatusEnum b : StatusEnum.values()) {
        if (b.value.equals(value)) {
          return b;
        }
      }
      throw new IllegalArgumentException("Unexpected value '" + value + "'");
    }
  }

  public static final String JSON_PROPERTY_STATUS = "status";
  @jakarta.annotation.Nullable
  private StatusEnum status;

  public static final String JSON_PROPERTY_COMPLETE = "complete";
  @jakarta.annotation.Nullable
  private Boolean complete = false;

  public Order() {
  }

  /**
   * Constructor with all args parameters
   */
  public Order(@JsonProperty(JSON_PROPERTY_ID) Long id, @JsonProperty(JSON_PROPERTY_PET_ID) Long petId, @JsonProperty(JSON_PROPERTY_QUANTITY) Integer quantity, @JsonProperty(JSON_PROPERTY_SHIP_DATE) OffsetDateTime shipDate, @JsonProperty(JSON_PROPERTY_STATUS) StatusEnum status, @JsonProperty(JSON_PROPERTY_COMPLETE) Boolean complete) {
    this.id = id;
    this.petId = petId;
    this.quantity = quantity;
    this.shipDate = shipDate;
    this.status = status;
    this.complete = complete;
  }

  public Order id(@jakarta.annotation.Nullable Long id) {
    
    this.id = id;
    return this;
  }

  /**
   * Get id
   * @return id
   */
  @jakarta.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_ID)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public Long getId() {
    return id;
  }


  @JsonProperty(JSON_PROPERTY_ID)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
  public void setId(@jakarta.annotation.Nullable Long id) {
    this.id = id;
  }

  public Order petId(@jakarta.annotation.Nullable Long petId) {
    
    this.petId = petId;
    return this;
  }

  /**
   * Get petId
   * @return petId
   */
  @jakarta.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_PET_ID)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public Long getPetId() {
    return petId;
  }


  @JsonProperty(JSON_PROPERTY_PET_ID)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
  public void setPetId(@jakarta.annotation.Nullable Long petId) {
    this.petId = petId;
  }

  public Order quantity(@jakarta.annotation.Nullable Integer quantity) {
    
    this.quantity = quantity;
    return this;
  }

  /**
   * Get quantity
   * @return quantity
   */
  @jakarta.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_QUANTITY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public Integer getQuantity() {
    return quantity;
  }


  @JsonProperty(JSON_PROPERTY_QUANTITY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
  public void setQuantity(@jakarta.annotation.Nullable Integer quantity) {
    this.quantity = quantity;
  }

  public Order shipDate(@jakarta.annotation.Nullable OffsetDateTime shipDate) {
    
    this.shipDate = shipDate;
    return this;
  }

  /**
   * Get shipDate
   * @return shipDate
   */
  @jakarta.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_SHIP_DATE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public OffsetDateTime getShipDate() {
    return shipDate;
  }


  @JsonProperty(JSON_PROPERTY_SHIP_DATE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
  public void setShipDate(@jakarta.annotation.Nullable OffsetDateTime shipDate) {
    this.shipDate = shipDate;
  }

  public Order status(@jakarta.annotation.Nullable StatusEnum status) {
    
    this.status = status;
    return this;
  }

  /**
   * Order Status
   * @return status
   */
  @jakarta.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_STATUS)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public StatusEnum getStatus() {
    return status;
  }


  @JsonProperty(JSON_PROPERTY_STATUS)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
  public void setStatus(@jakarta.annotation.Nullable StatusEnum status) {
    this.status = status;
  }

  public Order complete(@jakarta.annotation.Nullable Boolean complete) {
    
    this.complete = complete;
    return this;
  }

  /**
   * Get complete
   * @return complete
   */
  @jakarta.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_COMPLETE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public Boolean getComplete() {
    return complete;
  }


  @JsonProperty(JSON_PROPERTY_COMPLETE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)
  public void setComplete(@jakarta.annotation.Nullable Boolean complete) {
    this.complete = complete;
  }


  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Order order = (Order) o;
    return Objects.equals(this.id, order.id) &&
        Objects.equals(this.petId, order.petId) &&
        Objects.equals(this.quantity, order.quantity) &&
        Objects.equals(this.shipDate, order.shipDate) &&
        Objects.equals(this.status, order.status) &&
        Objects.equals(this.complete, order.complete);
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, petId, quantity, shipDate, status, complete);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Order {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    petId: ").append(toIndentedString(petId)).append("\n");
    sb.append("    quantity: ").append(toIndentedString(quantity)).append("\n");
    sb.append("    shipDate: ").append(toIndentedString(shipDate)).append("\n");
    sb.append("    status: ").append(toIndentedString(status)).append("\n");
    sb.append("    complete: ").append(toIndentedString(complete)).append("\n");
    sb.append("}");
    return sb.toString();
  }

  /**
   * Convert the given object to string with each line indented by 4 spaces
   * (except the first line).
   */
  private String toIndentedString(Object o) {
    if (o == null) {
      return "null";
    }
    return o.toString().replace("\n", "\n    ");
  }

}


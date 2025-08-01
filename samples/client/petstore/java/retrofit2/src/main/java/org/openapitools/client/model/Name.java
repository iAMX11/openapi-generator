/*
 * OpenAPI Petstore
 * This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\
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
import com.google.gson.TypeAdapter;
import com.google.gson.annotations.JsonAdapter;
import com.google.gson.annotations.SerializedName;
import com.google.gson.stream.JsonReader;
import com.google.gson.stream.JsonWriter;
import java.io.IOException;

/**
 * Model for testing model name same as property name
 */
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.15.0-SNAPSHOT")
public class Name {
  public static final String SERIALIZED_NAME_NAME = "name";
  @SerializedName(SERIALIZED_NAME_NAME)
  @javax.annotation.Nonnull
  private Integer name;

  public static final String SERIALIZED_NAME_SNAKE_CASE = "snake_case";
  @SerializedName(SERIALIZED_NAME_SNAKE_CASE)
  @javax.annotation.Nullable
  private Integer snakeCase;

  public static final String SERIALIZED_NAME_PROPERTY = "property";
  @SerializedName(SERIALIZED_NAME_PROPERTY)
  @javax.annotation.Nullable
  private String property;

  public static final String SERIALIZED_NAME_123NUMBER = "123Number";
  @SerializedName(SERIALIZED_NAME_123NUMBER)
  @javax.annotation.Nullable
  private Integer _123number;

  public Name() {
  }
  /**
   * Constructor with only readonly parameters
   */
  
  public Name(
     Integer snakeCase, 
     Integer _123number
  ) {
    this();
    this.snakeCase = snakeCase;
    this._123number = _123number;
  }

  public Name name(@javax.annotation.Nonnull Integer name) {
    
    this.name = name;
    return this;
  }

  /**
   * Get name
   * @return name
   */
  @javax.annotation.Nonnull

  public Integer getName() {
    return name;
  }


  public void setName(@javax.annotation.Nonnull Integer name) {
    this.name = name;
  }

  /**
   * Get snakeCase
   * @return snakeCase
   */
  @javax.annotation.Nullable

  public Integer getSnakeCase() {
    return snakeCase;
  }



  public Name property(@javax.annotation.Nullable String property) {
    
    this.property = property;
    return this;
  }

  /**
   * Get property
   * @return property
   */
  @javax.annotation.Nullable

  public String getProperty() {
    return property;
  }


  public void setProperty(@javax.annotation.Nullable String property) {
    this.property = property;
  }

  /**
   * Get _123number
   * @return _123number
   */
  @javax.annotation.Nullable

  public Integer get123number() {
    return _123number;
  }



  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Name name = (Name) o;
    return Objects.equals(this.name, name.name) &&
        Objects.equals(this.snakeCase, name.snakeCase) &&
        Objects.equals(this.property, name.property) &&
        Objects.equals(this._123number, name._123number);
  }

  @Override
  public int hashCode() {
    return Objects.hash(name, snakeCase, property, _123number);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Name {\n");
    sb.append("    name: ").append(toIndentedString(name)).append("\n");
    sb.append("    snakeCase: ").append(toIndentedString(snakeCase)).append("\n");
    sb.append("    property: ").append(toIndentedString(property)).append("\n");
    sb.append("    _123number: ").append(toIndentedString(_123number)).append("\n");
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


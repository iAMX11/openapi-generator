package org.openapitools.model;

import java.net.URI;
import java.util.Objects;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import org.openapitools.model.Animal;
import org.springframework.lang.Nullable;
import org.openapitools.jackson.nullable.JsonNullable;
import java.time.OffsetDateTime;
import javax.validation.Valid;
import javax.validation.constraints.*;


import java.util.*;
import javax.annotation.Generated;

/**
 * Cat
 */


@Generated(value = "org.openapitools.codegen.languages.SpringCodegen", comments = "Generator version: 7.15.0-SNAPSHOT")
public class Cat extends Animal {

  private @Nullable Boolean declawed;

  public Cat() {
    super();
  }

  /**
   * Constructor with only required parameters
   */
  public Cat(String className) {
    super(className);
  }

  public Cat declawed(@Nullable Boolean declawed) {
    this.declawed = declawed;
    return this;
  }

  /**
   * Get declawed
   * @return declawed
   */
  
  @ApiModelProperty(value = "")
  @JsonProperty("declawed")
  public @Nullable Boolean getDeclawed() {
    return declawed;
  }

  public void setDeclawed(@Nullable Boolean declawed) {
    this.declawed = declawed;
  }


  public Cat className(String className) {
    super.className(className);
    return this;
  }

  public Cat color(String color) {
    super.color(color);
    return this;
  }
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    Cat cat = (Cat) o;
    return Objects.equals(this.declawed, cat.declawed) &&
        super.equals(o);
  }

  @Override
  public int hashCode() {
    return Objects.hash(declawed, super.hashCode());
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class Cat {\n");
    sb.append("    ").append(toIndentedString(super.toString())).append("\n");
    sb.append("    declawed: ").append(toIndentedString(declawed)).append("\n");
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


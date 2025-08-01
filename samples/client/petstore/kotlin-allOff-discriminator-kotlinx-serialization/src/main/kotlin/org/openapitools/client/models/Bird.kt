/**
 *
 * Please note:
 * This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * Do not edit this file manually.
 *
 */

@file:Suppress(
    "ArrayInDataClass",
    "EnumEntryName",
    "RemoveRedundantQualifierName",
    "UnusedImport"
)

package org.openapitools.client.models

import org.openapitools.client.models.Animal

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.Contextual
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

/**
 * 
 *
 * @param id 
 * @param featherType 
 * @param optionalProperty 
 */
@Serializable

@SerialName(value = "BIRD")
data class Bird (

    @Contextual @SerialName(value = "id")
    override val id: java.util.UUID,

    @SerialName(value = "featherType")
    val featherType: kotlin.String,

    @Contextual @SerialName(value = "optional_property")
    override val optionalProperty: java.math.BigDecimal? = null

) : Animal() {


}


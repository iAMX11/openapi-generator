/**
 * OpenAPI Petstore
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * NOTE: This class is auto generated by OpenAPI-Generator 7.15.0-SNAPSHOT.
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

/*
 * SchemaWithSet_vaccinationBook.h
 *
 * Vaccination book of the pet
 */

#ifndef ORG_OPENAPITOOLS_CLIENT_MODEL_SchemaWithSet_vaccinationBook_H_
#define ORG_OPENAPITOOLS_CLIENT_MODEL_SchemaWithSet_vaccinationBook_H_


#include "CppRestPetstoreClient/ModelBase.h"

#include "CppRestPetstoreClient/model/Vaccine.h"
#include <set>

namespace org {
namespace openapitools {
namespace client {
namespace model {

class Vaccine;


/// <summary>
/// Vaccination book of the pet
/// </summary>
class  SchemaWithSet_vaccinationBook
    : public ModelBase
{
public:
    SchemaWithSet_vaccinationBook();
    virtual ~SchemaWithSet_vaccinationBook();

    /////////////////////////////////////////////
    /// ModelBase overrides

    void validate() override;

    web::json::value toJson() const override;
    bool fromJson(const web::json::value& json) override;

    void toMultipart(std::shared_ptr<MultipartFormData> multipart, const utility::string_t& namePrefix) const override;
    bool fromMultiPart(std::shared_ptr<MultipartFormData> multipart, const utility::string_t& namePrefix) override;


    /////////////////////////////////////////////
    /// SchemaWithSet_vaccinationBook members


    std::set<std::shared_ptr<Vaccine>> getVaccines() const;
    bool vaccinesIsSet() const;
    void unsetVaccines();
    void setVaccines(const std::set<std::shared_ptr<Vaccine>>& value);


protected:
    std::set<std::shared_ptr<Vaccine>> m_Vaccines;
    bool m_VaccinesIsSet;

};


}
}
}
}

#endif /* ORG_OPENAPITOOLS_CLIENT_MODEL_SchemaWithSet_vaccinationBook_H_ */

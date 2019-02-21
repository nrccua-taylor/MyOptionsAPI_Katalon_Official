<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Put_Checklist_MarkArchived</name>
   <tag></tag>
   <elementGuidId>1be4c86f-7d6c-4471-a6cc-b113d9c4c406</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;    {\n        \&quot;id\&quot;: 118,\n        \&quot;is_visible\&quot;: true,\n        \&quot;title\&quot;: \&quot;Start taking foreign language courses\&quot;,\n        \&quot;description\&quot;: \&quot;This will be great prep for the foreign language classes that will be required in high school.\&quot;,\n        \&quot;link\&quot;: \&quot;https://v74ej.app.goo.gl/NHvI5hSG3HaAFoyk1\&quot;,\n        \&quot;deadline\&quot;: null,\n        \&quot;high_school_grade\&quot;: 0,\n        \&quot;semester\&quot;: null,\n        \&quot;is_athlete\&quot;: false,\n        \&quot;is_first_gen\&quot;: false,\n        \&quot;created_at\&quot;: \&quot;2018-05-30T04:34:40.981Z\&quot;,\n        \&quot;updated_at\&quot;: \&quot;2018-06-28T04:12:43.243Z\&quot;,\n        \&quot;deleted_at\&quot;: null,\n        \&quot;status\&quot;: 2,\n        \&quot;status_updated_at\&quot;: null,\n        \&quot;personal_notes\&quot;: null\n    }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${G_API_URL_CHECKLIST_PUT}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.G_API_URL_CHECKLIST_PUT</defaultValue>
      <description></description>
      <id>f8674c9c-3e1b-4092-ac74-87887b0bdc7f</id>
      <masked>false</masked>
      <name>G_API_URL_CHECKLIST_PUT</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

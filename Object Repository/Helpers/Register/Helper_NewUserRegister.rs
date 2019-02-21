<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Helper_NewUserRegister</name>
   <tag></tag>
   <elementGuidId>295b8cf4-ec8c-4647-ba1e-42afb567d568</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;taylordbmerge+122@gmail.com\&quot;,\n  \&quot;password\&quot;: \&quot;Test12345\&quot;,\n  \&quot;first_name\&quot;: \&quot;AutoZ\&quot;,\n  \&quot;last_name\&quot;: \&quot;MationZ\&quot;,\n  \&quot;date_of_birth\&quot;: \&quot;1992-03-16T00:00:00.000Z\&quot;,\n  \&quot;tos_agreed_at\&quot;: \&quot;2019-01-22T11:25:14.000Z\&quot;,\n  \&quot;user_type\&quot;: \&quot;string\&quot;,\n  \&quot;email_feature_updates\&quot;: true,\n  \&quot;referral\&quot;: {\n    \&quot;type\&quot;: \&quot;object\&quot;,\n    \&quot;properties\&quot;: {\n      \&quot;id\&quot;: {\n        \&quot;type\&quot;: \&quot;integer\&quot;,\n        \&quot;example\&quot;: 1\n      }\n    }\n  }\n}-&quot;,
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${G_API_URL_SIGNUP}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.G_API_URL_SIGNUP</defaultValue>
      <description></description>
      <id>53df9001-9b9e-4fca-9e5b-a2579968f8d7</id>
      <masked>false</masked>
      <name>G_API_URL_SIGNUP</name>
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

assertThat(response.getStatusCode()).isEqualTo(200)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

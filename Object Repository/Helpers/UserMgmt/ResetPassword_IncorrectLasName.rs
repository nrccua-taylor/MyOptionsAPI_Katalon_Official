<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ResetPassword_IncorrectLasName</name>
   <tag></tag>
   <elementGuidId>b8f14207-29fd-4618-a5a8-f48ce5f2033f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;${G_SignUp_Student_Email}\&quot;,\n  \&quot;firstName\&quot;: \&quot;${G_SignUp_Student_FirstName}\&quot;,\n  \&quot;lastName\&quot;: \&quot;xxxxxxxxxxxxxx\&quot;\n}&quot;,
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
   <restUrl>${G_API_URL_resetPassword}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.G_API_URL_resetPassword</defaultValue>
      <description></description>
      <id>5f159004-b59c-463d-9abe-4040a1cf4a72</id>
      <masked>false</masked>
      <name>G_API_URL_resetPassword</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_resetPassword_Student_firstName</defaultValue>
      <description></description>
      <id>b9548b45-c734-4bb9-9119-26256a84e327</id>
      <masked>false</masked>
      <name>G_resetPassword_Student_firstName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_resetPassword_Student_Email</defaultValue>
      <description></description>
      <id>c0e2efc0-af5e-4205-ac0e-5124407237d6</id>
      <masked>false</masked>
      <name>G_resetPassword_Student_Email</name>
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



WS.verifyResponseStatusCode(response, 404)

assertThat(response.getStatusCode()).isEqualTo(404)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

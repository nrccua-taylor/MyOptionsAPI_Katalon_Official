<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ChangePassword_MissingLastName</name>
   <tag></tag>
   <elementGuidId>f465d4a5-f363-4096-ae7d-958da9f34453</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;${G_aosUser_Email}\&quot;,\n  \&quot;token\&quot;: \&quot;${G_aosUser_PasswordToken}\&quot;,\n  \&quot;password\&quot;: \&quot;${G_aosUser_NewPassword}\&quot;,\n  \&quot;firstName\&quot;: \&quot;${G_aosUser_FirstName)\&quot;\n}&quot;,
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
   <restUrl>${G_API_URL_changePassword}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.G_API_URL_changePassword</defaultValue>
      <description></description>
      <id>b82dbbbf-e751-49f5-b0b0-f1132a1f4a61</id>
      <masked>false</masked>
      <name>G_API_URL_changePassword</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_aosUser_Email</defaultValue>
      <description></description>
      <id>1a9db5e2-26c2-4ddd-9478-8fb825fa2cb7</id>
      <masked>false</masked>
      <name>G_aosUser_Email</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_aosUser_passwordToken</defaultValue>
      <description></description>
      <id>f0859af9-dd68-48d2-be15-54558cb6f6cf</id>
      <masked>false</masked>
      <name>G_aosUser_passwordToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_aosUser_newPassword</defaultValue>
      <description></description>
      <id>b5cdcab2-d884-45f9-90ea-ad1ea339879f</id>
      <masked>false</masked>
      <name>G_aosUser_newPassword</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_aosUser_firstName</defaultValue>
      <description></description>
      <id>b359c096-2425-487d-a904-04ac14d8c9d3</id>
      <masked>false</masked>
      <name>G_aosUser_firstName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_aosUser_lastName</defaultValue>
      <description></description>
      <id>dacbf881-4f26-4660-9e4c-316802274070</id>
      <masked>false</masked>
      <name>G_AOSuser_LastName</name>
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





WS.verifyResponseStatusCode(response, 400)

assertThat(response.getStatusCode()).isEqualTo(400)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Login</description>
   <name>Login-Bad_Password</name>
   <tag></tag>
   <elementGuidId>fdf5862e-92bf-4e3c-a39e-4950b2cfb37c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;username\&quot;:\&quot;${VAR_USER_LOGIN}\&quot;,\&quot;password\&quot;:\&quot;${VAR_USER_PASSWORD}\&quot;}&quot;,
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
   <restUrl>https://staging-api.myoptions.org/auth/local</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'taylordbmerge+1@gmail.com'</defaultValue>
      <description>MO Prod Username</description>
      <id>b7051eac-d1ae-454d-a8ab-14a4a6f8c324</id>
      <masked>false</masked>
      <name>VAR_USER_LOGIN</name>
   </variables>
   <variables>
      <defaultValue>'Test123'</defaultValue>
      <description>Bad Password</description>
      <id>3cab4eb0-4186-47cf-8037-095d44a08f0b</id>
      <masked>false</masked>
      <name>VAR_USER_PASSWORD</name>
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





WS.verifyResponseStatusCode(response, 401)

assertThat(response.getStatusCode()).isEqualTo(401)

assertThat(response.getResponseText()).contains('&quot;errorMessage&quot;:&quot;Incorrect password.&quot;')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Helper_MO_Login_IncorrectUsername</name>
   <tag></tag>
   <elementGuidId>24db7d2c-8688-4167-9d83-698250e309d0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;username\&quot;:\&quot;BadUserName\&quot;,\&quot;password\&quot;:\&quot;${G_Login_Password}\&quot;}&quot;,
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
   <restUrl>${G_API_URL_LOGIN}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.G_Login_Username_MO</defaultValue>
      <description></description>
      <id>d1d1b743-0532-4f55-ab60-8bd6b10018ba</id>
      <masked>false</masked>
      <name>G_Login_Username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_Login_Password_MO</defaultValue>
      <description></description>
      <id>5fe542ac-7758-4279-b28b-f47a424cf30f</id>
      <masked>false</masked>
      <name>G_Login_Password</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_API_URL_LOGIN</defaultValue>
      <description></description>
      <id>a91703c6-51d1-47f5-8de7-ad5bfc0e1b06</id>
      <masked>false</masked>
      <name>G_API_URL_LOGIN</name>
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

assertThat(response.getResponseText()).contains('errorMessage')





</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

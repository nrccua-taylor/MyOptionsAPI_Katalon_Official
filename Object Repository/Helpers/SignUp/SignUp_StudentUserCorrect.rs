<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SignUp_StudentUserCorrect</name>
   <tag></tag>
   <elementGuidId>5c486ed0-32ff-4ca1-80ac-95626a07a17c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;${G_SignUp_StudentCorrect_Email}\&quot;,\n  \&quot;password\&quot;: \&quot;Test12345\&quot;,\n  \&quot;first_name\&quot;: \&quot;${G_SignUp_StudentCorrect_FirstName}\&quot;,\n  \&quot;last_name\&quot;: \&quot;${G_SignUp_StudentCorrect_LastName}\&quot;\n}&quot;,
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
      <id>cf9ffa03-c9bf-4636-b1e9-849d016c7ba2</id>
      <masked>false</masked>
      <name>G_API_URL_SIGNUP</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_SignUp_StudentCorrect_FirstName</defaultValue>
      <description></description>
      <id>bdba6896-9fd2-45e3-aecc-45efa0fca1d6</id>
      <masked>false</masked>
      <name>G_SignUp_StudentCorrect_FirstName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_SignUp_StudentCorrect_Email</defaultValue>
      <description></description>
      <id>62bf7636-77e0-44bd-94cb-9e3c4dfff9f5</id>
      <masked>false</masked>
      <name>G_SignUp_StudentCorrect_Email</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_SignUp_StudentCorrect_LastName</defaultValue>
      <description></description>
      <id>f3390fc0-40ff-4db2-92ff-df2bfc35b7e2</id>
      <masked>false</masked>
      <name>G_SignUp_StudentCorrect_LastName</name>
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

<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get_User_TestScores</name>
   <tag></tag>
   <elementGuidId>d12946d8-bd2d-4997-b5a9-a75217b3ef96</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${G_ENVIRONMENT_URL}?/users/${G_studentID/${G_GetTestScores_URL_ENDPOINT}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.G_GetTestScores_URL_ENDPOINT</defaultValue>
      <description></description>
      <id>0aa0eb2b-46d7-42cc-909f-fa682fae3b3d</id>
      <masked>false</masked>
      <name>G_GetTestScores_URL_ENDPOINT</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_ENVIRONMENT_URL</defaultValue>
      <description></description>
      <id>0ceecea9-a59a-422b-8f7a-90e6a2692267</id>
      <masked>false</masked>
      <name>G_ENVIRONMENT_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_studentID</defaultValue>
      <description></description>
      <id>1d70e782-ff19-4ea3-a8c3-e17c7c14cbb5</id>
      <masked>false</masked>
      <name>G_studentID</name>
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

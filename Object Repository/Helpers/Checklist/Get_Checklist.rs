<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get_Checklist</name>
   <tag></tag>
   <elementGuidId>6f08b5c3-faea-485e-afcb-3ed291db4f0c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${G_ENVIRONMENT_URL}?/${G_studentID_TS}/${G_GetChecklist_URL_ENDPOINT}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.G_ENVIRONMENT_URL</defaultValue>
      <description></description>
      <id>f8674c9c-3e1b-4092-ac74-87887b0bdc7f</id>
      <masked>false</masked>
      <name>G_ENVIRONMENT_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_studentID_TS</defaultValue>
      <description></description>
      <id>dd50d3cf-bdb0-4edf-a36d-c11064c129ab</id>
      <masked>false</masked>
      <name>G_studentID_TS</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_GetChecklist_URL_ENDPOINT</defaultValue>
      <description></description>
      <id>0b3df0e2-f9fa-4bcc-bfaa-d8bd0e0ccf00</id>
      <masked>false</masked>
      <name>G_GetChecklist_URL_ENDPOINT</name>
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

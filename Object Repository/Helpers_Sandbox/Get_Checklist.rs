<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get_Checklist</name>
   <tag></tag>
   <elementGuidId>d10730a6-a173-4911-bab2-a61ebcf5edf2</elementGuidId>
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
      <id>2303ad47-2cfa-48e6-824a-08b65f17f7fc</id>
      <masked>false</masked>
      <name>G_ENVIRONMENT_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_studentID_TS</defaultValue>
      <description></description>
      <id>785d095c-4f96-4269-867f-27b42795da89</id>
      <masked>false</masked>
      <name>G_studentID_TS</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_GetChecklist_URL_ENDPOINT</defaultValue>
      <description></description>
      <id>7570cb1a-66f4-4d5c-b1d8-5f528fa560f4</id>
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

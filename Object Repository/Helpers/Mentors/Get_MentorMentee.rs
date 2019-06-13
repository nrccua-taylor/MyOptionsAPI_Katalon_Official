<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get_MentorMentee</name>
   <tag></tag>
   <elementGuidId>b428b264-7d2e-4c4b-8632-7aab061c6ae0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;mentor_email\&quot;: \&quot;${G_TimeStampMentorEmailAddress}\&quot;,\n  \&quot;invite_status\&quot;: 1,\n  \&quot;mentor_name\&quot;: \&quot;Mentor Name Pending Teacher\&quot;,\n  \&quot;mentor_type\&quot;: 2\n}&quot;,
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
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${G_ENVIRONMENT_URL}/users/${G_studentID_TS}/mentors?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.G_ENVIRONMENT_URL</defaultValue>
      <description></description>
      <id>6ea81339-08be-446f-a542-44c9f9bd5012</id>
      <masked>false</masked>
      <name>G_ENVIRONMENT_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_studentID_TS</defaultValue>
      <description></description>
      <id>e745b022-b579-4183-ae02-30e43406dbee</id>
      <masked>false</masked>
      <name>G_studentID_TS</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_TimeStampMentorEmailAddress</defaultValue>
      <description></description>
      <id>6487de57-b9dc-43a4-a5be-630b73d022d7</id>
      <masked>false</masked>
      <name>G_TimeStampMentorEmailAddress</name>
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

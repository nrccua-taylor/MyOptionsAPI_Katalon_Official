<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post_Mentor_AcceptedCoach</name>
   <tag></tag>
   <elementGuidId>d4e9c498-7820-4cf2-96f1-da2f17bfa8b6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;mentor_email\&quot;: \&quot;${G_TimeStampMentorEmailAddress}\&quot;,\n  \&quot;invite_status\&quot;: 2,\n  \&quot;mentor_name\&quot;: \&quot;Mentor Name Accepted Coach\&quot;,\n  \&quot;mentor_type\&quot;: 6\n}&quot;,
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





WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

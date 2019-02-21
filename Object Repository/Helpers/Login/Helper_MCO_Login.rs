<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Helper_MCO_Login</name>
   <tag></tag>
   <elementGuidId>c1a752a9-3e00-4505-a049-505593e8ebd9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;username\&quot;:\&quot;${G_Login_Username}\&quot;,\&quot;password\&quot;:\&quot;${G_Login_Password}\&quot;}&quot;,
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
      <defaultValue>GlobalVariable.G_Login_Username_MCO</defaultValue>
      <description></description>
      <id>d1d1b743-0532-4f55-ab60-8bd6b10018ba</id>
      <masked>false</masked>
      <name>G_Login_Username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_Login_Password_MCO</defaultValue>
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


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)






assertThat(response.getResponseText()).contains('id')


assertThat(response.getResponseText()).contains('student_key')


assertThat(response.getResponseText()).contains('created_at')


assertThat(response.getResponseText()).contains('updated_at')


assertThat(response.getResponseText()).contains('username')


assertThat(response.getResponseText()).contains('email')


assertThat(response.getResponseText()).contains('school_id')


assertThat(response.getResponseText()).contains('email_verified_at')


assertThat(response.getResponseText()).contains('is_school_verified')


assertThat(response.getResponseText()).contains('bearer_token')


assertThat(response.getResponseText()).contains('update_increment')


assertThat(response.getResponseText()).contains('facebook_id')


assertThat(response.getResponseText()).contains('twitter_id')


assertThat(response.getResponseText()).contains('is_profile_public')


assertThat(response.getResponseText()).contains('tos_agreed_at')


assertThat(response.getResponseText()).contains('gpa')


assertThat(response.getResponseText()).contains('admittedly_score')


assertThat(response.getResponseText()).contains('is_tutorial_completed')


assertThat(response.getResponseText()).contains('tutorial_step')


assertThat(response.getResponseText()).contains('preferences')


assertThat(response.getResponseText()).contains('personal_color')


assertThat(response.getResponseText()).contains('profile')


assertThat(response.getResponseText()).contains('profile_completion')


assertThat(response.getResponseText()).contains('personal_story')


assertThat(response.getResponseText()).contains('subscription_expires_at')


assertThat(response.getResponseText()).contains('referred_by_code')


assertThat(response.getResponseText()).contains('date_of_birth')


assertThat(response.getResponseText()).contains('from_mo')


assertThat(response.getResponseText()).contains('old_password_format')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

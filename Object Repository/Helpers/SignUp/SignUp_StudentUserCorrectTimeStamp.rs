<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SignUp_StudentUserCorrectTimeStamp</name>
   <tag></tag>
   <elementGuidId>da871d54-f070-462c-b5de-57b0ddfe6267</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;${G_TimeStampEmailAddress}\&quot;,\n  \&quot;password\&quot;: \&quot;Test12345\&quot;,\n  \&quot;first_name\&quot;: \&quot;${G_signUp_studentCorrect_firstName}\&quot;,\n  \&quot;last_name\&quot;: \&quot;${G_signUp_studentCorrect_lastName}\&quot;\n}&quot;,
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
      <name>G_signUp_studentCorrect_firstName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_TimeStampEmailAddress</defaultValue>
      <description></description>
      <id>62bf7636-77e0-44bd-94cb-9e3c4dfff9f5</id>
      <masked>false</masked>
      <name>G_TimeStampEmailAddress</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.G_SignUp_StudentCorrect_LastName</defaultValue>
      <description></description>
      <id>f3390fc0-40ff-4db2-92ff-df2bfc35b7e2</id>
      <masked>false</masked>
      <name>G_signUp_studentCorrect_lastName</name>
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




//verify the response code
WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)




//verify the response schema
assertThat(response.getResponseText()).contains('id')
assertThat(response.getResponseText()).contains('student_key')
assertThat(response.getResponseText()).contains('created_at')
assertThat(response.getResponseText()).contains('updated_at')
WS.verifyElementPropertyValue(response, 'username', GlobalVariable.G_TimeStampEmailAddress)
WS.verifyElementPropertyValue(response, 'email', GlobalVariable.G_TimeStampEmailAddress)
assertThat(response.getResponseText()).contains('school_id')
assertThat(response.getResponseText()).contains('email_verified_at')
assertThat(response.getResponseText()).contains('is_school_verified')
assertThat(response.getResponseText()).contains('update_increment')
assertThat(response.getResponseText()).contains('facebook_id')
assertThat(response.getResponseText()).contains('twitter_id')
assertThat(response.getResponseText()).contains('is_profile_public')
assertThat(response.getResponseText()).contains('tos_agreed_at')
assertThat(response.getResponseText()).contains('gpa')
assertThat(response.getResponseText()).contains('admittedly_score')
assertThat(response.getResponseText()).contains('is_tutorial_completed')
assertThat(response.getResponseText()).contains('bearer_token')
assertThat(response.getResponseText()).contains('tutorial_step')
assertThat(response.getResponseText()).contains('preferences')
assertThat(response.getResponseText()).contains('notification_method')
assertThat(response.getResponseText()).contains('email_feature_updates')
assertThat(response.getResponseText()).contains('gpa_scale')
assertThat(response.getResponseText()).contains('personal_color')
assertThat(response.getResponseText()).contains('profile')
assertThat(response.getResponseText()).contains('address_one')
assertThat(response.getResponseText()).contains('address_two')
assertThat(response.getResponseText()).contains('first_name')
assertThat(response.getResponseText()).contains('last_name')
//WS.verifyElementPropertyValue(response, 'first_name', GlobalVariable.G_signUp_studentCorrect_firstName)
//WS.verifyElementPropertyValue(response, 'last_name', GlobalVariable.G_signUp_studentCorrect_lastName)
assertThat(response.getResponseText()).contains('is_valid_address')
assertThat(response.getResponseText()).contains('user_type')
assertThat(response.getResponseText()).contains('is_survey_complete')
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

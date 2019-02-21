<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Login</description>
   <name>Login-MCO_USER</name>
   <tag></tag>
   <elementGuidId>d2d31650-8330-44d2-bbb0-28f744b7e16f</elementGuidId>
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
      <defaultValue>'Fridayone1'</defaultValue>
      <description>MO Prod Username</description>
      <id>b7051eac-d1ae-454d-a8ab-14a4a6f8c324</id>
      <masked>false</masked>
      <name>VAR_USER_LOGIN</name>
   </variables>
   <variables>
      <defaultValue>'Test12345'</defaultValue>
      <description>MO Prod Password</description>
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


assertThat(response.getResponseText()).contains('old_password_format')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

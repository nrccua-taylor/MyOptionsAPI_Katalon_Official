<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update_User_SXSW</name>
   <tag></tag>
   <elementGuidId>15b70b9a-7ff3-4eb9-a676-adbbf49c33a4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;:180500,\n  \&quot;student_key\&quot;:\&quot;964741243\&quot;,\n  \&quot;created_at\&quot;:\&quot;2018-11-26T15:24:49.000Z\&quot;,\n  \&quot;updated_at\&quot;:\&quot;2018-12-06T20:13:36.000Z\&quot;,\n  \&quot;username\&quot;:\&quot;sablowskysxsw@gmail.com\&quot;,\n  \&quot;email\&quot;:\&quot;sablowskysxsw@gmail.com\&quot;,\n  \&quot;school_id\&quot;:null,\n  \&quot;email_verified_at\&quot;:null,\n  \&quot;is_school_verified\&quot;:false,\n  \&quot;bearer_token\&quot;:\&quot;QQ6C6APQ2W1MSKT8H2GVYU5M\&quot;,\n  \&quot;update_increment\&quot;:8,\n  \&quot;facebook_id\&quot;:null,\n  \&quot;twitter_id\&quot;:\&quot;\&quot;,\n  \&quot;is_profile_public\&quot;:false,\n  \&quot;tos_agreed_at\&quot;:\&quot;2018-11-26T15:24:55.884Z\&quot;,\n  \&quot;gpa\&quot;:2.8,\n  \&quot;admittedly_score\&quot;:0,\n  \&quot;is_tutorial_completed\&quot;:false,\n  \&quot;tutorial_step\&quot;:\&quot;\&quot;,\n  \&quot;preferences\&quot;:{\n    \&quot;notification_method\&quot;:\&quot;email\&quot;,\n    \&quot;email_feature_updates\&quot;:\&quot;true\&quot;,\n    \&quot;gpa_scale\&quot;:\&quot;AF\&quot;\n  },\n  \&quot;personal_color\&quot;:null,\n  \&quot;profile\&quot;:{\n    \&quot;address_one\&quot;:\&quot;13401 Legendary Dr\&quot;,\n    \&quot;address_two\&quot;:\&quot;Apt 4301\&quot;,\n    \&quot;first_name\&quot;:\&quot;Third\&quot;,\n    \&quot;last_name\&quot;:\&quot;Try\&quot;,\n    \&quot;gender\&quot;:\&quot;M\&quot;,\n    \&quot;phone\&quot;:\&quot;2013910081\&quot;,\n    \&quot;zip_code\&quot;:\&quot;78727\&quot;,\n    \&quot;state\&quot;:\&quot;TX\&quot;,\n    \&quot;city\&quot;:\&quot;Austin\&quot;,\n    \&quot;country\&quot;:\&quot;United States of America\&quot;,\n    \&quot;is_valid_address\&quot;:true,\n    \&quot;user_type\&quot;:\&quot;high-school\&quot;,\n    \&quot;graduation_year\&quot;:\&quot;2020\&quot;,\n    \&quot;is_survey_complete\&quot;:\&quot;false\&quot;,\n    \&quot;is_onboarding_complete\&quot;:\&quot;true\&quot;,\n    \&quot;college_start_year\&quot;:\&quot;2020\&quot;\n  },\n  \&quot;profile_completion\&quot;:0.75,\n  \&quot;personal_story\&quot;:{\n    \&quot;high_school_type\&quot;:[\n      \&quot;ADVPLACE\&quot;,\n      \&quot;OTHERADVAN\&quot;],\n    \&quot;activity\&quot;:[\n      \&quot;HONORSCLUB\&quot;,\n      \&quot;WRITING\&quot;],\n    \&quot;military_interests\&quot;:[\n      \&quot;NOMILITARY\&quot;],\n    \&quot;parents_went_college\&quot;:\&quot;false\&quot;,\n    \&quot;life_after_college\&quot;:[\n      \&quot;AGRICULTURE\&quot;,\n      \&quot;ENGINEERING\&quot;,\n      \&quot;NEWSMEDIA\&quot;],\n    \&quot;racial_ethnic\&quot;:[\n      \&quot;WHITERACE\&quot;],\n    \&quot;info_materials\&quot;:[\n      \&quot;GAPYEARPGM\&quot;,\n      \&quot;TUTORINFO\&quot;],\n    \&quot;school_start_method\&quot;:\&quot;direct\&quot;\n  },\n  \&quot;subscription_expires_at\&quot;:null,\n  \&quot;referred_by_code\&quot;:null,\n  \&quot;date_of_birth\&quot;:\&quot;1992-03-16T00:00:00.000Z\&quot;,\n  \&quot;from_mo\&quot;:false,\n  \&quot;old_password_format\&quot;:false\n}\n&quot;,
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
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://staging-api.myoptions.org/users/180500</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
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

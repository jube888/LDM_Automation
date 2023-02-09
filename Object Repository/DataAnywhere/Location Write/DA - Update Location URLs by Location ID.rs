<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DA - Update Location URLs by Location ID</name>
   <tag></tag>
   <elementGuidId>52729774-010e-425f-98df-b9ed616ce7c4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;LocationId\&quot;:\&quot;${GlobalVariable.LocationID}\&quot;,\&quot;LocationNumber\&quot;:\&quot;${GlobalVariable.LocationNumber}\&quot;,\&quot;Data\&quot;:{\&quot;appointmenturl1\&quot;:\&quot;https://test.com/ApptTest\&quot;,\&quot;couponurl1\&quot;:\&quot;https://test.com/CouponTest\&quot;,\&quot;couponurl2\&quot;:\&quot;https://test.com/Coupon\&quot;,\&quot;couponurl3\&quot;:\&quot;https://test.com/Coupon\&quot;,\&quot;couponurl4\&quot;:\&quot;https://test.com/Coupon\&quot;,\&quot;facebookurl\&quot;:\&quot;https://test.com/FBTest\&quot;,\&quot;instagramurl\&quot;:\&quot;https://test.com/InstaTest\&quot;,\&quot;inventorysearchurl1\&quot;:\&quot;https://test.com/InventoryTest\&quot;,\&quot;inventorysearchurl2\&quot;:\&quot;https://test.com/Inventory\&quot;,\&quot;inventorysearchurl3\&quot;:\&quot;https://test.com/Inventory\&quot;,\&quot;linkedinurl\&quot;:\&quot;https://test.com/LinkedINTest\&quot;,\&quot;menuurl1\&quot;:\&quot;https://test.com/MenuTest\&quot;,\&quot;menuurl2\&quot;:\&quot;https://test.com/Menu\&quot;,\&quot;orderaheadurl1\&quot;:\&quot;https://test.com/OrderTest\&quot;,\&quot;pinteresturl\&quot;:\&quot;https://test.com/PintTest\&quot;,\&quot;reservationsurl1\&quot;:\&quot;https://test.com/ResTest\&quot;,\&quot;reservationsurl2\&quot;:\&quot;https://test.com/Reservation\&quot;,\&quot;socialnetworkurl\&quot;:\&quot;https://test.com/SocialTest\&quot;,\&quot;twitterurl\&quot;:\&quot;https://test.com/TwitterTest\&quot;,\&quot;videourl\&quot;:\&quot;https://test.com/VidTest\&quot;,\&quot;virtualcareurl1\&quot;:\&quot;https://test.com/VirtualTest\&quot;,\&quot;virtualcareurl2\&quot;:\&quot;https://test.com/Insta\&quot;,}}&quot;,
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
      <webElementGuid>75b6e6ec-9f87-4ad0-b445-5578d67e9595</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Ocp-Apim-Subscription-Key</name>
      <type>Main</type>
      <value>${GlobalVariable.DASubKeyLocationWrite}</value>
      <webElementGuid>cd4e5433-51a5-4a84-89dd-644412c699c2</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://${GlobalVariable.baseDA}/locations-api-write/locations/LocationId/${GlobalVariable.LocationID}/LocationURLs</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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

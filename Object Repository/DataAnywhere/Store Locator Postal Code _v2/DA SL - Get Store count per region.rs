<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DA SL - Get Store count per region</name>
   <tag></tag>
   <elementGuidId>564e4d40-d5fc-4eb1-bf95-be67910166fe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Ocp-Apim-Subscription-Key</name>
      <type>Main</type>
      <value>${GlobalVariable.DASubKeyStoreLocatorV2}</value>
      <webElementGuid>6ea9b487-7c21-4f61-93bb-6dd13c4ea02f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${GlobalVariable.baseDA}/v2/api/stores/regions/count?filter={&quot;CountryCode&quot;:&quot;CA&quot;, &quot;Name&quot;:&quot;Test Short&quot;, &quot;Products&quot;:&quot;XX1&quot;, &quot;Products&quot;:&quot;XX2&quot;, &quot;Services&quot;:&quot;YY1&quot;, &quot;Services&quot;:&quot;YY2&quot;, &quot;Services&quot;:&quot;YY3&quot;, &quot;Brands&quot;:&quot;TT1&quot;, &quot;Brands&quot;:&quot;TT2&quot;}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseDA</defaultValue>
      <description></description>
      <id>45339d80-9ae1-4dbd-a2e2-5d30deb67d10</id>
      <masked>false</masked>
      <name>baseDA</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DASubKeyStoreLocatorV2</defaultValue>
      <description></description>
      <id>61e52b61-8e33-468a-b3db-3348ac5fce89</id>
      <masked>false</masked>
      <name>DASubKeyStoreLocatorV2</name>
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

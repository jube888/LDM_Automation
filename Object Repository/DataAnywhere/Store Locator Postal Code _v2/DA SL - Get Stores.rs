<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DA SL - Get Stores</name>
   <tag></tag>
   <elementGuidId>a3488e21-5b90-448b-9198-38f014b04b07</elementGuidId>
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
      <webElementGuid>e703a161-bdfa-4438-a9d8-bc974e3da3ed</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${GlobalVariable.baseDA}/v2/api/stores?latitude=44.315495&amp;longitude=-79.54704&amp;distance=500&amp;units=miles&amp;maxItems=100&amp;sort=1&amp;filter={&quot;CountryCode&quot;:&quot;CA&quot;, &quot;Name&quot;:&quot;Test Short&quot;, &quot;Products&quot;:&quot;XX1&quot;, &quot;Products&quot;:&quot;XX2&quot;, &quot;Services&quot;:&quot;YY1&quot;, &quot;Services&quot;:&quot;YY2&quot;, &quot;Services&quot;:&quot;YY3&quot;, &quot;Brands&quot;:&quot;TT1&quot;, &quot;Brands&quot;:&quot;TT2&quot;}</restUrl>
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
      <id>e52a08dd-f6a8-4051-9484-71156789da49</id>
      <masked>false</masked>
      <name>baseDA</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DASubKeyStoreLocatorV2</defaultValue>
      <description></description>
      <id>cb18e979-fc21-4794-b9dc-c7cb7b608c71</id>
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

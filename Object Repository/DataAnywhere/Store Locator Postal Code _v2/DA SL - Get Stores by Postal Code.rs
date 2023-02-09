<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DA SL - Get Stores by Postal Code</name>
   <tag></tag>
   <elementGuidId>0d76e388-c1aa-444e-94e5-2dde00ba552e</elementGuidId>
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
      <webElementGuid>d102f958-ceaa-4182-82d1-f0eda34404ec</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${GlobalVariable.baseDA}/v2/api/stores/postalcode?filter={&quot;CountryCode&quot;:&quot;CA&quot;, &quot;Name&quot;:&quot;Testing Business&quot;, &quot;Products&quot;:&quot;Test1&quot;, &quot;Products&quot;:&quot;Test2&quot;, &quot;Services&quot;:&quot;Test1&quot;, &quot;Services&quot;:&quot;Test2&quot;, &quot;Services&quot;:&quot;Test3&quot;, &quot;Brands&quot;:&quot;Test1&quot;, &quot;Brands&quot;:&quot;Test2&quot;}&amp;postalCode=L9S 2C8</restUrl>
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
      <id>bd677e0d-ce9e-4071-ace0-0c734fc41410</id>
      <masked>false</masked>
      <name>baseDA</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DASubKeyStoreLocatorV2</defaultValue>
      <description></description>
      <id>bfc5696c-fea4-4a7c-90cc-b71269593e83</id>
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

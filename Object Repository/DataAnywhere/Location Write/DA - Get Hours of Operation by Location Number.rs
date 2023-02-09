<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DA - Get Hours of Operation by Location Number</name>
   <tag></tag>
   <elementGuidId>8c56e670-97a9-4e7b-87f5-edecffeb4637</elementGuidId>
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
      <value>${GlobalVariable.DASubKeyLocationWrite}</value>
      <webElementGuid>4d4c0307-1e59-4f8c-9a5d-a9492d9a23f3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${GlobalVariable.baseDA}/locations-api-write/locations/LocationNumber/${GlobalVariable.LocationNumber}/Hours</restUrl>
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
      <id>4eb7d523-e598-47a9-8b86-29cbce7d8acc</id>
      <masked>false</masked>
      <name>baseDA</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LocationNumber</defaultValue>
      <description></description>
      <id>9c6f322d-2427-4b89-9a4a-bdbee164ef37</id>
      <masked>false</masked>
      <name>LocationNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DASubKey</defaultValue>
      <description></description>
      <id>075340ad-10ed-4cd8-8acc-f33ba3ebd70a</id>
      <masked>false</masked>
      <name>DASubKey</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

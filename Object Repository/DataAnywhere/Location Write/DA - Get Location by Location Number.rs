<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DA - Get Location by Location Number</name>
   <tag></tag>
   <elementGuidId>6fff9bbe-d354-4c6f-8252-ac2abefe2f2e</elementGuidId>
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
      <webElementGuid>f976975d-b017-4d24-bc17-a08272d4ee40</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${GlobalVariable.baseDA}/locations-api-write/location?LocationNumber=${GlobalVariable.LocationNumber}</restUrl>
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
      <id>51428248-8e61-4192-931d-933fecdc3a64</id>
      <masked>false</masked>
      <name>baseDA</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LocationNumber</defaultValue>
      <description></description>
      <id>9e500164-061b-422f-b346-f1cdb6ee7604</id>
      <masked>false</masked>
      <name>LocationNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DASubKey</defaultValue>
      <description></description>
      <id>2befcc65-f498-436a-aab1-a899c3cbf0df</id>
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

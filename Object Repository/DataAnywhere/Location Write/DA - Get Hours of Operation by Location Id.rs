<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DA - Get Hours of Operation by Location Id</name>
   <tag></tag>
   <elementGuidId>91277b33-13fb-4fe4-947d-62db4d48ac85</elementGuidId>
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
      <webElementGuid>da6c99de-d0e7-4a8f-bac9-eccf5109bc2f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${GlobalVariable.baseDA}/locations-api-write/locations/LocationId/${GlobalVariable.LocationID}/Hours</restUrl>
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
      <id>4ecf4abc-6e3b-47c2-aa3d-8c3669d0c298</id>
      <masked>false</masked>
      <name>baseDA</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LocationID</defaultValue>
      <description></description>
      <id>89f1e633-3165-46cd-94c6-a45aae93925b</id>
      <masked>false</masked>
      <name>LocationID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DASubKey</defaultValue>
      <description></description>
      <id>6011ab10-d496-42c8-9362-14f60b0ac098</id>
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

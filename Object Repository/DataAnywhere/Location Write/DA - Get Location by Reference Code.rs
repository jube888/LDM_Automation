<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DA - Get Location by Reference Code</name>
   <tag></tag>
   <elementGuidId>553924ae-06f3-4e2d-9a45-14ccb5916833</elementGuidId>
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
      <webElementGuid>6f5fcfc2-aa32-48fe-a6bd-6062ab8636bf</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${GlobalVariable.baseDA}/locations-api-write/location?ReferenceCode=${GlobalVariable.ReferenceCode}</restUrl>
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
      <id>cfc21d5e-e580-444e-9a9a-fdc078f58890</id>
      <masked>false</masked>
      <name>baseDA</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ReferenceCode</defaultValue>
      <description></description>
      <id>54ecfe08-dcf0-4db3-9380-1de1d6257167</id>
      <masked>false</masked>
      <name>ReferenceCode</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DASubKey</defaultValue>
      <description></description>
      <id>1915b345-b786-4eb0-a092-37b59b9cbe47</id>
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

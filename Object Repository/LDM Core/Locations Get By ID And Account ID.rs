<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Locations Get By ID And Account ID</name>
   <tag></tag>
   <elementGuidId>c2ea150e-d45e-4cfe-b7a6-73960cab7621</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-api-key</name>
      <type>Main</type>
      <value>${GlobalVariable.xapikey}</value>
      <webElementGuid>fe1b3b61-1683-4a4c-bebb-7ce695476334</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${GlobalVariable.baseLDMCore}/api/Locations/${GlobalVariable.ID}?accountID=${GlobalVariable.AccountID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseLDMCore</defaultValue>
      <description></description>
      <id>8152f1db-9219-4bc2-a991-cec73394dfbf</id>
      <masked>false</masked>
      <name>baseLDMCore</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LocationID</defaultValue>
      <description></description>
      <id>380a8215-6461-41a5-94d7-b3d02c5bdb61</id>
      <masked>false</masked>
      <name>LocationID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.AccountID</defaultValue>
      <description></description>
      <id>6c2c3c37-5c52-42ee-8f67-b9a08889f8ea</id>
      <masked>false</masked>
      <name>AccountID</name>
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

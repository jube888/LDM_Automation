import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper as JsonSlurper
import groovy.json.JsonOutput as JsonOutput

GlobalVariable.DASubKeyStoreLocatorV2 = '0646791860bd47ea910b44633da2d3a7'
println(GlobalVariable.DASubKeyStoreLocatorV2)

response = WS.sendRequest(findTestObject('DataAnywhere/Store Locator Postal Code _v2/Store Fields'))

WS.verifyResponseStatusCode(response, 401)

println(response.getResponseBodyContent())
// def newError = response.getResponseBodyContent()

// GlobalVariable.Error = newError

assert response.getResponseText().contains("Access denied due to invalid subscription key. Make sure to provide a valid key for an active subscription")


if (GlobalVariable.baseDA == "dataanywherebeta.azure-api.net") {
	GlobalVariable.DASubKeyStoreLocatorV2 = "33040c1dd9f642958b818ab76f7f2565"
} else {
	GlobalVariable.DASubKeyStoreLocatorV2 = "ProdSubKey"
}

println(GlobalVariable.DASubKeyStoreLocatorV2)

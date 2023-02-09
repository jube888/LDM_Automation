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

response = WS.sendRequest(findTestObject('DataAnywhere/Location Write/DA - Get Locations with Status = 1 or Deleted'))

WS.verifyResponseStatusCode(response, 200)

def jsonSlurper = new JsonSlurper()

def JSONResponse = jsonSlurper.parseText(response.getResponseBodyContent())

GlobalVariable.Location = JSONResponse

println(GlobalVariable.Location)

JSONArray = JSONResponse.Data

println(JSONArray)

for(item in JSONArray) {
	if(item.Status == "Deleted") {
	println ("Location Number " + item.LocationNumber + " has Status 1 or Deleted")
}
	else { assert item.Status == "Deleted"
		println ("Location Number " + item.LocationNumber + " does not have Status 1 or Deleted")
}
}
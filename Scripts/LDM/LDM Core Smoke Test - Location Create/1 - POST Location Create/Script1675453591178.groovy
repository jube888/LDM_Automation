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

CustomKeywords.'randomNumber.randomNumGenerator.random_number'(0)

response = WS.sendRequest(findTestObject('LDM Core/Location Create - POST Copy', [('baseLDMCore') : GlobalVariable.baseLDMCore
            , ('ResellerIDStoreLocatorV2') : GlobalVariable.ResellerIDStoreLocatorV2, ('AccountIDStoreLocatorV2') : GlobalVariable.AccountIDStoreLocatorV2
            , ('AutoReferenceCode') : GlobalVariable.AutoReferenceCode]))

WS.verifyResponseStatusCode(response, 200)

println(response.getResponseBodyContent())

def jsonSlurper = new JsonSlurper()

def JSONResponse = jsonSlurper.parseText(response.getResponseBodyContent())

println(JSONResponse.Data)

GlobalVariable.ID = JSONResponse.Data

response2 = WS.sendRequestAndVerify(findTestObject('LDM Core/Location by ID'))

// def jsonSlurper = new JsonSlurper()

def jsonSlurper2 = new JsonSlurper()

def JSONResponse2 = jsonSlurper2.parseText(response2.getResponseBodyContent())

GlobalVariable.Location = JSONResponse2

println(GlobalVariable.Location)

def locationJSON2 = GlobalVariable.Location

def test2 = locationJSON2

println(test2)

ID = test2.ID

LocationNumber = test2.LocationNumber

ClientRefID = test2.ClientRefID

ResellerID = test2.ResellerID

AccountID = test2.AccountID

Modified = test2.Modified

GlobalVariable.ID = ID

GlobalVariable.LocationNumber = LocationNumber

GlobalVariable.ClientRefID = ClientRefID

GlobalVariable.ResellerID = ResellerID

GlobalVariable.AccountID = AccountID

GlobalVariable.Modified = Modified

println(GlobalVariable.ID)

println(GlobalVariable.LocationNumber)

println(GlobalVariable.ClientRefID)

println(GlobalVariable.ResellerID)

println(GlobalVariable.AccountID)

println(GlobalVariable.Modified)


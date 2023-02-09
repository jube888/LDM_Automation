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

response = WS.sendRequest(findTestObject('LDM Core/LocationSave', [('Ref') : GlobalVariable.AutoReferenceCode]))

def jsonSlurper = new JsonSlurper()

def JSONResponse = jsonSlurper.parseText(response.getResponseBodyContent())

GlobalVariable.Location = JSONResponse.Data

println(GlobalVariable.Location)

def locationJSON = GlobalVariable.Location

def test = jsonSlurper.parseText(locationJSON)

println(test)

ID = test.ID

LocationNumber = test.LocationNumber

ClientRefID = test.ClientRefID

ResellerID = test.ResellerID

AccountID = test.AccountID

Modified = test.Modified

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


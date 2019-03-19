import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import groovy.time.TimeCategory as TimeCategory

//Creating a local variable to store the email address + the nanotime of system 
String TimeStampEmailAddress = ('nrccua.welcomeemail+' + System.nanoTime()) + '@gmail.com'

// print the email address out for reference if needed, prints in console
System.out.println(TimeStampEmailAddress)

//set the local variable to a global variable for the timestamp email email address
GlobalVariable.G_TimeStampEmailAddress = TimeStampEmailAddress

//execute the test with the timestamp email address. 
WS.sendRequestAndVerify(findTestObject('Helpers/SignUp/SignUp_StudentUserCorrectTimeStamp', [('G_API_URL_SIGNUP') : GlobalVariable.G_API_URL_SIGNUP
            , ('G_signUp_studentCorrect_firstName') : GlobalVariable.G_signUp_studentCorrect_firstName, ('G_TimeStampEmailAddress') : GlobalVariable.G_TimeStampEmailAddress
            , ('G_signUp_studentCorrect_lastName') : GlobalVariable.G_signUp_studentCorrect_lastName]))


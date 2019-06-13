import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.testobject.TestObjectProperty as TestObjectProperty
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import groovy.time.TimeCategory as TimeCategory

//Creating a local variable to store the email address + the nanotime of system 
String TimeStampMentorEmailAddress = ('nrccua.test' + System.nanoTime()) + '@nrccua.org'

// print the email address out for reference if needed, prints in console
System.out.println(TimeStampMentorEmailAddress)

//set the local variable to a global variable for the timestamp email email address
GlobalVariable.G_TimeStampMentorEmailAddress = TimeStampMentorEmailAddress

// load test request object which will use token above in Authorization
RequestObject getUserInfoTestObject = findTestObject('Helpers/Mentors/Post_Mentor_PendingFamily', [('G_ENVIRONMENT_URL') : GlobalVariable.G_ENVIRONMENT_URL, ('G_studentID_TS') : GlobalVariable.G_studentID_TS
            , ('G_TimeStampMentorEmailAddress') : GlobalVariable.G_TimeStampMentorEmailAddress])

// if getUserInfoTestObject HTTP headers have no Authorization item
getUserInfoTestObject.getHttpHeaderProperties().add(new TestObjectProperty('Authorization', ConditionType.EQUALS, 'Bearer ' + 
    GlobalVariable.G_BearerToken_TS))

// THIRD STEP send the request and run verifications
WS.sendRequestAndVerify(getUserInfoTestObject)


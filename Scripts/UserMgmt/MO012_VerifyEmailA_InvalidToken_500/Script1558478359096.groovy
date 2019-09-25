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

WS.sendRequestAndVerify(findTestObject('Helpers/UserMgmt/VerifyEmail_IncorrectToken', [('G_API_URL_verifyEmail') : GlobalVariable.G_API_URL_verifyEmail
            , ('G_AOSuser_Email') : GlobalVariable.G_aosUser_Email, ('G_AOSuser_PasswordToken') : GlobalVariable.G_aosUser_passwordToken
            , ('G_AOSuser_NewPassword') : GlobalVariable.G_aosUser_newPassword, ('G_AOSuser_FirstName') : GlobalVariable.G_aosUser_firstName
            , ('G_AOSuser_LastName') : GlobalVariable.G_aosUser_lastName]))


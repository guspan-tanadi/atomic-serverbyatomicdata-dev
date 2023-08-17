import { test, expect } from '@playwright/test';
import {
  signIn,
  newDrive,
  newResource,
  waitForCommit,
  editTitle,
  editableTitle,
  getCurrentSubject,
  makeDrivePublic,
  openNewSubjectWindow,
  timestamp,
  before,
} from './test-utils';
test.describe('documents', async () => {
  test.beforeEach(before);

  test('create document, edit, page title, websockets', async ({
    page,
    browser,
  }) => {
    await signIn(page);
    await newDrive(page);
    await makeDrivePublic(page);
    // Create a document
    await newResource('document', page);
    // commit for saving initial document
    await waitForCommit(page);
    // commit for initializing the first element (paragraph)
    await waitForCommit(page);
    const title = `Document ${timestamp()}`;
    await editTitle(title, page);

    await page.press(editableTitle, 'Enter');

    const teststring = `My test: ${timestamp()}`;
    await page.fill('textarea', teststring);
    await waitForCommit(page);

    // commit editing paragraph
    await expect(page.locator(`text=${teststring}`)).toBeVisible();

    // multi-user
    const currentUrl = await getCurrentSubject(page);
    await page.waitForTimeout(1000);
    const page2 = await openNewSubjectWindow(browser, currentUrl!);
    await expect(page2.locator(`text=${teststring}`)).toBeVisible();
    expect(await page2.title()).toEqual(title);

    // Add a new line on first page, check if it appears on the second
    await page.keyboard.press('Enter');
    await waitForCommit(page);
    await waitForCommit(page);
    const syncText = 'New paragraph';
    await page.keyboard.type(syncText);
    // If this fails to show up, websockets aren't working properly
    await expect(page2.locator(`text=${syncText}`)).toBeVisible();
  });
});

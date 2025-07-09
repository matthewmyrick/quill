import * as vscode from 'vscode';
import * as path from 'path';

export function activate(context: vscode.ExtensionContext) {

	console.log('Congratulations, your extension "quill-lang" is now active!');

	const treeDataProvider = new QuillNotebooksProvider(context.extensionUri);
	vscode.window.registerTreeDataProvider('quill-notebooks-view', treeDataProvider);

    // Register the "Add Notebook" command
	context.subscriptions.push(
        vscode.commands.registerCommand('quill.addNotebook', () => {
            // For now, just show a message. Later, this could open an input box.
            vscode.window.showInformationMessage('Add Notebook command triggered!');
        })
    );

    // Register the "Add Page" command
    context.subscriptions.push(
        vscode.commands.registerCommand('quill.addPage', (notebookItem: vscode.TreeItem) => {
            // The `notebookItem` is the tree item the command was executed on.
            vscode.window.showInformationMessage(`Add Page command triggered for: ${notebookItem.label}`);
        })
    );
}

export function deactivate() {}

class QuillNotebooksProvider implements vscode.TreeDataProvider<vscode.TreeItem> {
  private extensionUri: vscode.Uri;

  constructor(extensionUri: vscode.Uri) {
    this.extensionUri = extensionUri;
  }

  getTreeItem(element: vscode.TreeItem): vscode.TreeItem {
    return element;
  }

  getChildren(element?: vscode.TreeItem): Thenable<vscode.TreeItem[]> {
    if (!element) {
      const notebook1 = new vscode.TreeItem('My Project Notebook', vscode.TreeItemCollapsibleState.Collapsed);
      const notebook2 = new vscode.TreeItem('Personal Ideas', vscode.TreeItemCollapsibleState.Collapsed);

      notebook1.iconPath = {
        light: vscode.Uri.joinPath(this.extensionUri, 'images', 'icon.svg'),
        dark: vscode.Uri.joinPath(this.extensionUri, 'images', 'icon.svg')
      };
      // Set a context value to identify this item as a notebook
      notebook1.contextValue = 'notebook';

      notebook2.iconPath = {
        light: vscode.Uri.joinPath(this.extensionUri, 'images', 'icon.svg'),
        dark: vscode.Uri.joinPath(this.extensionUri, 'images', 'icon.svg')
      };
      // Set a context value to identify this item as a notebook
      notebook2.contextValue = 'notebook';


      return Promise.resolve([notebook1, notebook2]);
    }

    if (element.label === 'My Project Notebook') {
        const page1 = new vscode.TreeItem('Project Tasks', vscode.TreeItemCollapsibleState.None);
        const page2 = new vscode.TreeItem('Meeting Notes', vscode.TreeItemCollapsibleState.None);

        page1.iconPath = {
            light: vscode.Uri.joinPath(this.extensionUri, 'images', 'page-icon.svg'),
            dark: vscode.Uri.joinPath(this.extensionUri, 'images', 'page-icon.svg')
        };
        // Pages don't need a context value for now, unless we want to add actions to them.
        
        page2.iconPath = {
            light: vscode.Uri.joinPath(this.extensionUri, 'images', 'page-icon.svg'),
            dark: vscode.Uri.joinPath(this.extensionUri, 'images', 'page-icon.svg')
        };

        return Promise.resolve([page1, page2]);
    }

    return Promise.resolve([]);
  }
}
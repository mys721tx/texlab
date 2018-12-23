package texlab.completion.latex

import org.eclipse.lsp4j.CompletionItem
import texlab.LatexDocument
import texlab.completion.CompletionProvider
import texlab.completion.CompletionRequest
import texlab.contains
import texlab.syntax.latex.LatexCommandSyntax

abstract class LatexCommandProvider : CompletionProvider {

    override fun getItems(request: CompletionRequest): List<CompletionItem> {
        return if (request.document is LatexDocument) {
            val command = request.document
                    .tree
                    .root
                    .descendants()
                    .lastOrNull { it.range.contains(request.position) }

            if (command is LatexCommandSyntax) {
                getItems(request, command)
            } else {
                listOf()
            }
        } else {
            listOf()
        }
    }

    protected abstract fun getItems(request: CompletionRequest, command: LatexCommandSyntax): List<CompletionItem>
}

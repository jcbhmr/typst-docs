<script setup>
import Category from ".vitepress/theme/components/Category.vue"
import Func from ".vitepress/theme/components/Func.vue"
import Group from ".vitepress/theme/components/Group.vue"
import Html from ".vitepress/theme/components/Html.vue"
import Packages from ".vitepress/theme/components/Packages.vue"
import Symbols from ".vitepress/theme/components/Symbols.vue"
import Type from ".vitepress/theme/components/Type.vue"
</script>

<Category v-if="$params.page.body.kind === 'category'" :page="$params.page" :category="$params.page.body.content" />
<Func v-if="$params.page.body.kind === 'func'" :page="$params.page" :func="$params.page.body.content" />
<Group v-if="$params.page.body.kind === 'group'" :page="$params.page" :group="$params.page.body.content" />
<Html v-if="$params.page.body.kind === 'html'" :page="$params.page" :html="$params.page.body.content" />
<Packages v-if="$params.page.body.kind === 'packages'" :page="$params.page" :packages="$params.page.body.content" />
<Symbols v-if="$params.page.body.kind === 'symbols'" :page="$params.page" :symbols="$params.page.body.content" />
<Type v-if="$params.page.body.kind === 'type'" :page="$params.page" :type="$params.page.body.content" />

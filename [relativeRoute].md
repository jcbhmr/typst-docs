<script setup>
import Category from "@/components/Category.vue"
import Func from "@/components/Func.vue"
import Group from "@/components/Group.vue"
import Html from "@/components/Html.vue"
import Packages from "@/components/Packages.vue"
import Symbols from "@/components/Symbols.vue"
import Type from "@/components/Type.vue"
</script>

<Category v-if="$params.page.body.kind === 'category'" :page="$params.page" :category="$params.page.body.content" />
<Func v-if="$params.page.body.kind === 'func'" :page="$params.page" :func="$params.page.body.content" />
<Group v-if="$params.page.body.kind === 'group'" :page="$params.page" :group="$params.page.body.content" />
<Html v-if="$params.page.body.kind === 'html'" :page="$params.page" :html="$params.page.body.content" />
<Packages v-if="$params.page.body.kind === 'packages'" :page="$params.page" :packages="$params.page.body.content" />
<Symbols v-if="$params.page.body.kind === 'symbols'" :page="$params.page" :symbols="$params.page.body.content" />
<Type v-if="$params.page.body.kind === 'type'" :page="$params.page" :type="$params.page.body.content" />

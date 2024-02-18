<script setup>
const details = {{ category.details.as_str()|json }}
</script>

# {{ category.title }}

<div v-html="details"></div>

## {{ locale.translations.definitions }}

{% for item in category.items %}- **[{{ item.name }}]({{ item.route }}):** {{ item.oneliner }}{% endfor %}

{% if let Some(shorthands) = category.shorthands %}
## {{ locale.translations.shorthands }}

{{ locale.translations.shorthands_description|safe }}

{% if !shorthands.markup.is_empty() %}
### {{ locale.translations.within_markup_mode }}

{% for symbol in shorthands.markup %}- **{{ symbol.name }}:** TODO{% endfor %}
{% endif %}

{% if !shorthands.math.is_empty() %}
### {{ locale.translations.within_math_mode }}

{% for symbol in shorthands.math %}- **{{ symbol.name }}:** TODO{% endfor %}
{% endif %}

{% endif %}
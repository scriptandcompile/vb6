package io.proleap.vb6.asg.statement.property.set;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;

import java.io.File;

import org.junit.Test;

import io.proleap.vb6.VbTestBase;
import io.proleap.vb6.asg.metamodel.Arg;
import io.proleap.vb6.asg.metamodel.Module;
import io.proleap.vb6.asg.metamodel.Program;
import io.proleap.vb6.asg.metamodel.Variable;
import io.proleap.vb6.asg.metamodel.call.Call;
import io.proleap.vb6.asg.metamodel.call.Call.CallType;
import io.proleap.vb6.asg.metamodel.call.VariableCall;
import io.proleap.vb6.asg.metamodel.statement.Statement;
import io.proleap.vb6.asg.metamodel.statement.StatementTypeEnum;
import io.proleap.vb6.asg.metamodel.statement.let.Let;
import io.proleap.vb6.asg.metamodel.statement.property.set.PropertySet;
import io.proleap.vb6.asg.metamodel.type.VbBaseType;
import io.proleap.vb6.asg.runner.impl.VbParserRunnerImpl;

public class PropertySetTest extends VbTestBase {

	@Test
	public void test() throws Exception {
		final File inputFile = new File("src/test/resources/io/proleap/vb6/asg/statement/property/set/PropertySet.cls");
		final Program program = new VbParserRunnerImpl().analyzeFile(inputFile);

		final Module module = program.getClazzModule("PropertySet");
		final Variable variable = module.getVariable("CurrentI");
		assertNotNull(variable);

		final PropertySet propertySet = (PropertySet) module.getStatements().get(0);
		assertNotNull(propertySet);
		assertEquals(StatementTypeEnum.PROPERTY_SET, propertySet.getStatementType());
		assertEquals(1, propertySet.getArgs().size());
		assertEquals(1, propertySet.getStatements().size());

		{
			final Arg arg = propertySet.getArgsList().get(0);
			assertEquals("I", arg.getName());
			assertEquals(VbBaseType.INTEGER, arg.getType());
		}

		{
			final Statement statement = propertySet.getStatements().get(0);
			assertEquals(StatementTypeEnum.LET, statement.getStatementType());

			final Let let = (Let) statement;

			{
				final Call leftHandCall = let.getLeftHandCall();
				assertEquals(CallType.VARIABLE_CALL, leftHandCall.getCallType());

				final VariableCall variableCall = (VariableCall) leftHandCall.unwrap();
				assertEquals(variable, variableCall.getVariable());
			}
		}
	}
}
